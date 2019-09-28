#include <signal.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
/*****************************************************************************
 * Run programs on Linux with resource limited.
 * Based on setrlimit.(https://linux.die.net/man/2/setrlimit)
 * Author: Ke Shen
 * Modified By 9646516
 * Argvs:
    + argv[1]: Time limit(cpu time): milliseconds
    + argv[2]: Memory limit: bytes
    + argv[3]: Output limit: bytes
    + argv[4]: Stack limit: bytes
    + argv[5]: Input sourcefile
    + argv[6]: Output sourcefile
    + argv[7]: Answer sourcefile
    + argv[8]: Running arguments
    + argv[9]: Checker sourcefile
 * EXITCODE:
    + 152 = 24 = CPU TIME EXCEEDED
    + 134 = 6 = MLE( Aborted )
 * Debug:
    printf( "%d %d %s %ld\n" , WEXITSTATUS(status) , status_code , strsignal( status_code ) , result.ru_maxrss );
 *****************************************************************************/
long long timelimit;
int Exceeded_wall_clock_time;
char checker_arguments[666];
const int judge_user = 6666;
__pid_t pid;

void errExit(char *msg) {
    fprintf(stdout, "{\"result\":\"%s\", \"additional_info\": \"%s\" }\n", "Judger Error", msg);
    exit(-1);
}

void goodExit(char *msg, long long timecost, long long memorycost) {
    fprintf(stdout, "{\"result\":\"%s\", \"time_cost\": %lld , \"memory_cost\": %lld }\n", msg, timecost, memorycost);
    exit(0);
}

void set_limit(int type, int value, int ext) {
    struct rlimit _;
    _.rlim_cur = (value);
    _.rlim_max = value + ext;
    if (setrlimit(type, &_) != 0)
        errExit("Setrlimit error");
}

void wait_to_kill_childprocess() {
    sleep(((timelimit + 999) / 1000) << 1);
    kill(pid, 9);
    Exceeded_wall_clock_time = 1;
}

int get_status_code(int x) {
    if (x > 128)
        x -= 128;
    return x;
}

int main(int argc, char *argv[]) {
    if (argc != 10)
        errExit("Arguments number should be 9");
    timelimit = atoll(argv[1]);
    long long memorylimit = atoll(argv[2]);
    long long outputlimit = atoll(argv[3]);
    long long stacklimit = atoll(argv[4]);
    char *input_sourcefile = argv[5];
    char *output_sourcefile = argv[6];
    char *answer_sourcefile = argv[7];
    char *running_arguments = argv[8];
    char *checker_sourcefile = argv[9];
    sprintf(checker_arguments, "%s %s %s %s", checker_sourcefile, input_sourcefile, output_sourcefile, answer_sourcefile);
    if (freopen("/dev/null", "w", stderr) == NULL)
        errExit("Can not redirect stderr");
    pid = fork();
    if (pid > 0) {
        pthread_t watch_thread;
        if (pthread_create(&watch_thread, NULL, (void *)wait_to_kill_childprocess, NULL))
            errExit("Can not create watch pthread");
        int status;
        struct rusage result;
        wait4(pid, &status, 0, &result);
        int status_code = get_status_code(WEXITSTATUS(status));
        if (status_code == -1)
            errExit("Unknown Error");
        else if (status_code == 127)
            errExit("Can not run target program( command not found )");
        long long timecost = (long long)result.ru_utime.tv_sec * 1000000LL + (long long)result.ru_utime.tv_usec;
        if (status_code == SIGXCPU || timecost > timelimit * 1000LL || Exceeded_wall_clock_time)
            goodExit("Time Limit Exceeded", timelimit, result.ru_maxrss);
        else if (status_code == SIGXFSZ)
            goodExit("Output Limit Exceeded", timecost / 1000, result.ru_maxrss);
        else if (result.ru_maxrss * 1024 > memorylimit || status_code == SIGIOT)
            goodExit("Memory Limit Exceeded", timecost / 1000, memorylimit / 1024);
        else if (status_code != 0)
            goodExit("Runtime Error", timecost / 1000, result.ru_maxrss);
        int checker_statuscode = system(checker_arguments);
        if (checker_statuscode == 0)
            goodExit("Accepted", timecost / 1000, result.ru_maxrss);
        else if (checker_statuscode > 256)
            errExit("Checker error");
        goodExit("Wrong Answer", timecost / 1000, result.ru_maxrss);
    } else if (pid == 0) {
        if (freopen(input_sourcefile, "r", stdin) == NULL)
            errExit("Can not redirect stdin");
        else if (freopen(output_sourcefile, "w", stdout) == NULL)
            errExit("Can not redirect stdout");
        else if (setuid(judge_user))
            errExit("Can not set uid");
        else {
            set_limit(RLIMIT_CPU, (timelimit + 999) / 1000, 1);
            set_limit(RLIMIT_DATA, memorylimit, 0);
            set_limit(RLIMIT_FSIZE, outputlimit, 0);
            set_limit(RLIMIT_STACK, stacklimit, 0);
            execl("/bin/sh", "sh", "-c", running_arguments, (char *)0);
        }
    } else
        errExit("Can not fork the child process");
    return 0;
}
