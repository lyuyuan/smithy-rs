package software.amazon.smithy.rust.codegen.smithy.generators

import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.protocoltests.traits.HttpRequestTestCase
import software.amazon.smithy.protocoltests.traits.HttpRequestTestsTrait
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.lang.rustBlock
import software.amazon.smithy.rust.codegen.lang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.util.dq

val DenyList = setOf(
    "RestJsonListsSerializeNull"
)

class HttpProtocolTestGenerator(
    private val protocolConfig: ProtocolConfig,
    private val operationShape: OperationShape,
    private val inputShape: StructureShape,
    private val writer: RustWriter
) {
    fun render() {
        operationShape.getTrait(HttpRequestTestsTrait::class.java).map {
            renderHttpRequestTests(it)
        }
    }

    private fun renderHttpRequestTests(httpRequestTestsTrait: HttpRequestTestsTrait) {
        with(protocolConfig) {
            writer.write("#[cfg(test)]")
            val operationName = symbolProvider.toSymbol(operationShape).name
            val testModuleName = "${operationName.toSnakeCase()}_request_test"
            writer.withModule(testModuleName) {
                httpRequestTestsTrait.testCases.filter { it.protocol == protocol }.filter { !DenyList.contains(it.id) }.forEach { testCase ->
                    renderHttpRequestTestCase(testCase, this)
                }
            }
        }
    }

    private val instantiator = with(protocolConfig) {
        Instantiator(symbolProvider, model, runtimeConfig)
    }

    private fun renderHttpRequestTestCase(httpRequestTestCase: HttpRequestTestCase, testModuleWriter: RustWriter) {
        httpRequestTestCase.documentation.map {
            testModuleWriter.setNewlinePrefix("/// ").write(it).write(httpRequestTestCase.id).setNewlinePrefix("")
        }
        testModuleWriter.write("#[test]")
        testModuleWriter.rustBlock("fn test_${httpRequestTestCase.id.toSnakeCase()}()") {
            writeInline("let input =")
            instantiator.render(httpRequestTestCase.params, inputShape, this)
            write(";")
            write("let http_request = input.build_http_request().body(()).unwrap();")
            checkQueryParams(this, httpRequestTestCase.queryParams)
            checkForbidQueryParams(this, httpRequestTestCase.forbidQueryParams)
            checkRequiredQueryParams(this, httpRequestTestCase.requireQueryParams)
            checkHeaders(this, httpRequestTestCase.headers)
            checkBody(this, httpRequestTestCase.body.orElse(""), httpRequestTestCase.bodyMediaType.orElse(null))
            with(httpRequestTestCase) {
                write(
                    """
                    assert_eq!(http_request.method(), ${method.dq()});
                    assert_eq!(http_request.uri().path(), ${uri.dq()});
                """
                )
                // TODO: assert on the body contents
                write("/* BODY:\n ${body.orElse("[ No Body ]")} */")
            }
        }
    }

    private fun checkBody(rustWriter: RustWriter, body: String, mediaType: String?) {
        if (body == "") {
            rustWriter.write("// No body")
            rustWriter.write("assert_eq!(input.build_body(), \"\");")
        } else {
            check(mediaType != null)
            checked(rustWriter) {
                rustWriter.addImport(RuntimeType.Std("str::FromStr").toSymbol(), null)
                write(
                    "\$T(input.build_body(), ${body.dq()}, \$T::from_str(${mediaType.dq()}).unwrap())",
                    RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "validate_body"),
                    RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "MediaType")
                )
            }
        }
    }

    private fun checked(rustWriter: RustWriter, inner: RustWriter.() -> Unit) {
        rustWriter.withBlock(
            "\$T(", ");", conditional = true,
            args = *arrayOf(RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "check"))
        ) {
            inner(this)
        }
    }

    private fun checkHeaders(rustWriter: RustWriter, headers: Map<String, String>) {
        if (headers.isEmpty()) {
            return
        }
        val variableName = "expected_headers"
        rustWriter.withBlock("let $variableName = &[", "];") {
            write(
                headers.entries.joinToString(",") {
                    "(${it.key.dq()}, ${it.value.dq()})"
                }
            )
        }
        rustWriter.write(
            "assert_eq!(\$T(&http_request, $variableName), Ok(()));",
            RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "validate_headers")
        )
    }

    private fun checkRequiredQueryParams(
        rustWriter: RustWriter,
        requiredParams: List<String>
    ) = basicCheck(requiredParams, rustWriter, "required_params", "require_query_params")

    private fun checkForbidQueryParams(
        rustWriter: RustWriter,
        forbidParams: List<String>
    ) = basicCheck(forbidParams, rustWriter, "forbid_params", "forbid_query_params")

    private fun checkQueryParams(
        rustWriter: RustWriter,
        queryParams: List<String>
    ) = basicCheck(queryParams, rustWriter, "expected_query_params", "validate_query_string")

    private fun basicCheck(
        params: List<String>,
        rustWriter: RustWriter,
        variableName: String,
        checkFunction: String
    ) {
        if (params.isEmpty()) {
            return
        }
        rustWriter.withBlock("let $variableName = ", ";") {
            strSlice(this, params)
        }
        rustWriter.write(
            "assert_eq!(\$T(&http_request, $variableName), Ok(()));",
            RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, checkFunction)
        )
    }

    private fun strSlice(writer: RustWriter, args: List<String>) {
        writer.withBlock("&[", "]") {
            write(args.joinToString(",") { it.dq() })
        }
    }
}