// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Mq {
    fn submit(&self, o: ::grpc::RequestOptions, p: super::MQ::submit_request) -> ::grpc::SingleResponse<super::MQ::submit_response>;

    fn check(&self, o: ::grpc::RequestOptions, p: super::MQ::check_request) -> ::grpc::SingleResponse<super::MQ::check_response>;
}

// client

pub struct MqClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_submit: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::MQ::submit_request, super::MQ::submit_response>>,
    method_check: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::MQ::check_request, super::MQ::check_response>>,
}

impl ::grpc::ClientStub for MqClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MqClient {
            grpc_client: grpc_client,
            method_submit: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/mq.Mq/submit".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_check: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/mq.Mq/check".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Mq for MqClient {
    fn submit(&self, o: ::grpc::RequestOptions, p: super::MQ::submit_request) -> ::grpc::SingleResponse<super::MQ::submit_response> {
        self.grpc_client.call_unary(o, p, self.method_submit.clone())
    }

    fn check(&self, o: ::grpc::RequestOptions, p: super::MQ::check_request) -> ::grpc::SingleResponse<super::MQ::check_response> {
        self.grpc_client.call_unary(o, p, self.method_check.clone())
    }
}

// server

pub struct MqServer;


impl MqServer {
    pub fn new_service_def<H : Mq + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/mq.Mq",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/mq.Mq/submit".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.submit(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/mq.Mq/check".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.check(o, p))
                    },
                ),
            ],
        )
    }
}
