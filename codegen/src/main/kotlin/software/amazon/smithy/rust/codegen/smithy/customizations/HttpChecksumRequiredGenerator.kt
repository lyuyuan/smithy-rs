/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.customizations

import software.amazon.smithy.codegen.core.CodegenException
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.traits.HttpChecksumRequiredTrait
import software.amazon.smithy.rust.codegen.rustlang.CargoDependency
import software.amazon.smithy.rust.codegen.rustlang.Writable
import software.amazon.smithy.rust.codegen.rustlang.asType
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.rustlang.writable
import software.amazon.smithy.rust.codegen.smithy.customize.OperationCustomization
import software.amazon.smithy.rust.codegen.smithy.customize.OperationSection
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.generators.operationBuildError
import software.amazon.smithy.rust.codegen.util.hasStreamingMember
import software.amazon.smithy.rust.codegen.util.hasTrait
import software.amazon.smithy.rust.codegen.util.inputShape

class HttpChecksumRequiredGenerator(
    private val protocolConfig: ProtocolConfig,
    private val operationShape: OperationShape
) : OperationCustomization() {
    override fun section(section: OperationSection): Writable {
        if (!operationShape.hasTrait<HttpChecksumRequiredTrait>()) {
            return emptySection
        }
        if (operationShape.inputShape(protocolConfig.model).hasStreamingMember(protocolConfig.model)) {
            throw CodegenException("HttpChecksum required cannot be applied to a streaming shape")
        }
        return when (section) {
            is OperationSection.MutateRequest -> writable {
                rustTemplate(
                    """
                    ${section.request} = ${section.request}.augment(|mut req, _| {
                        let data = req
                            .body()
                            .bytes()
                            .ok_or_else(||#{BuildError}::SerializationError(
                                "checksum can only be computed for non-streaming operations".into())
                            )?;
                        let checksum = #{md5}::compute(data);
                        req.headers_mut().insert(
                            #{http}::header::HeaderName::from_static("content-md5"),
                            format!("{:x}", checksum).parse().expect("checksum is valid header value")
                        );
                        Result::<_, #{BuildError}>::Ok(req)
                    })?;
                """,
                    "md5" to CargoDependency.Md5.asType(),
                    "http" to CargoDependency.Http.asType(),
                    "BuildError" to protocolConfig.runtimeConfig.operationBuildError()
                )
            }
            else -> emptySection
        }
    }
}
