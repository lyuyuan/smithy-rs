/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rustsdk.customize.s3

import software.amazon.smithy.aws.traits.protocols.RestXmlTrait
import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.model.shapes.ShapeId
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.transform.ModelTransformer
import software.amazon.smithy.rust.codegen.rustlang.CargoDependency
import software.amazon.smithy.rust.codegen.rustlang.Writable
import software.amazon.smithy.rust.codegen.rustlang.asType
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.rustBlockTemplate
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.rustlang.writable
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.customize.RustCodegenDecorator
import software.amazon.smithy.rust.codegen.smithy.generators.LibRsCustomization
import software.amazon.smithy.rust.codegen.smithy.generators.LibRsSection
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.letIf
import software.amazon.smithy.rust.codegen.smithy.protocols.ProtocolMap
import software.amazon.smithy.rust.codegen.smithy.protocols.RestXml
import software.amazon.smithy.rust.codegen.smithy.protocols.RestXmlFactory
import software.amazon.smithy.rust.codegen.smithy.traits.S3UnwrappedXmlOutputTrait
import software.amazon.smithy.rustsdk.AwsRuntimeType

/**
 * Top level decorator for S3
 * */
class S3Decorator : RustCodegenDecorator {
    override val name: String = "S3ExtendedError"
    override val order: Byte = 0

    private fun applies(serviceId: ShapeId) =
        serviceId == ShapeId.from("com.amazonaws.s3#AmazonS3")

    override fun protocols(serviceId: ShapeId, currentProtocols: ProtocolMap): ProtocolMap {
        return currentProtocols.letIf(applies(serviceId)) {
            it + mapOf(
                RestXmlTrait.ID to RestXmlFactory { protocolConfig ->
                    S3(protocolConfig)
                }
            )
        }
    }

    override fun libRsCustomizations(
        protocolConfig: ProtocolConfig,
        baseCustomizations: List<LibRsCustomization>
    ): List<LibRsCustomization> {
        return baseCustomizations.letIf(applies(protocolConfig.serviceShape.id)) {
            it + S3PubUse()
        }
    }

    override fun transformModel(service: ServiceShape, model: Model): Model {
        return model.letIf(applies(service.id)) {
            ModelTransformer.create().mapShapes(model) { shape ->
                // Apply the S3UnwrappedXmlOutput customization to GetBucketLocation (more
                // details on the S3UnwrappedXmlOutputTrait)
                if (shape is StructureShape && shape.id == ShapeId.from("com.amazonaws.s3#GetBucketLocationOutput")) {
                    shape.toBuilder().addTrait(S3UnwrappedXmlOutputTrait()).build()
                } else {
                    shape
                }
            }
        }
    }
}

class S3(protocolConfig: ProtocolConfig) : RestXml(protocolConfig) {
    private val runtimeConfig = protocolConfig.runtimeConfig
    override fun parseGenericError(operationShape: OperationShape): RuntimeType {
        return RuntimeType.forInlineFun("parse_generic_error", "xml_deser") {
            it.rustBlockTemplate(
                "pub fn parse_generic_error(response: &#{Response}<#{Bytes}>) -> Result<#{Error}, #{XmlError}>",
                "Response" to RuntimeType.http.member("Response"),
                "Bytes" to RuntimeType.Bytes,
                "Error" to RuntimeType.GenericError(runtimeConfig),
                "XmlError" to CargoDependency.smithyXml(runtimeConfig).asType().member("decode::XmlError")
            ) {
                rustTemplate(
                    """
                    if response.body().is_empty() {
                        let mut err = #{Error}::builder();
                        if response.status().as_u16() == 404 {
                            err.code("NotFound");
                        }
                        Ok(err.build())
                    } else {
                        let base_err = #{base_errors}::parse_generic_error(response.body().as_ref())?;
                        Ok(#{s3_errors}::parse_extended_error(base_err, &response))
                    }
                    """,
                    "base_errors" to restXmlErrors,
                    "s3_errors" to AwsRuntimeType.S3Errors,
                    "Error" to RuntimeType.GenericError(runtimeConfig)
                )
            }
        }
    }
}

class S3PubUse : LibRsCustomization() {
    override fun section(section: LibRsSection): Writable = when (section) {
        is LibRsSection.Body -> writable { rust("pub use #T::ErrorExt;", AwsRuntimeType.S3Errors) }
        else -> emptySection
    }
}
