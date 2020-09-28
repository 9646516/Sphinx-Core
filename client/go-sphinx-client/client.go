package go_sphinx_client

import (
	"context"
	_ "github.com/Shopify/sarama"
	"io"
)

type SphinxClient interface {
	PostSubmissionRequest(
		ctx context.Context, in *SphinxPostSubmissionRequest) (
		out *SphinxPostSubmissionReply, err error)
}

type SphinxPostSubmissionRequest struct {
	Code              io.Reader
	ProblemConfigPath string
	Language          uint64
	SubmissionID      uint64
}

type SphinxPostSubmissionReply struct {
	Partition int32
	Offset    int64
}

type IClientSender interface {
	SphinxClient
	End()
}

type IClient interface {
	SphinxClient
	Begin() IClientSender
}
