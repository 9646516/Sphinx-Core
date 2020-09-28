package go_sphinx_client

import (
	"context"
	_ "github.com/Shopify/sarama"
	"io"
)

type SphinxClient interface {
	PostSubmission(
		ctx context.Context, in *SphinxPostSubmissionRequest) (
		out *SphinxPostSubmissionReply, err error)
}

type SphinxEventHandler interface {
	GetSubmissionResult(in *GetSubmissionResultRequest) (
		out *GetSubmissionResultReply, err error)
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

type GetSubmissionResultRequest struct {
	JudgeStatus  JudgeStatus
	Key          []byte
	Mem          uint64
	Time         uint64
	SubmissionID uint64
	Last         uint32
	Score        uint64
	Info         []byte
}

type GetSubmissionResultReply struct {
}

type IClientSender interface {
	SphinxClient
	End()
}

type IClient interface {
	SphinxClient
	Begin() IClientSender
}
