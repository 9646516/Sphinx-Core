package go_sphinx_client

type JudgeStatus int
const (

	JudgeStatusAccepted JudgeStatus = iota
	JudgeStatusWrongAnswer
	JudgeStatusTimeLimitedError
	JudgeStatusRuntimeError
	JudgeStatusMemoryLimitedError
	JudgeStatusOutputLimitedError
	JudgeStatusCompileError
	JudgeStatusAssertFailed
	JudgeStatusUnknownError
)

var judgeStatusMapping = map[string]JudgeStatus {
	"ACCEPTED": JudgeStatusAccepted,
	"WRONG ANSWER": JudgeStatusWrongAnswer,
	"TIME LIMITED ERROR": JudgeStatusTimeLimitedError,
	"RUNTIME ERROR": JudgeStatusRuntimeError,
	"MEMORY LIMITED ERROR": JudgeStatusMemoryLimitedError,
	"OUTPUT LIMITED ERROR": JudgeStatusOutputLimitedError,
	"COMPILE ERROR": JudgeStatusCompileError,
	"ASSERT FAILED": JudgeStatusAssertFailed,
	"UNKNOWN ERROR": JudgeStatusUnknownError,
}
