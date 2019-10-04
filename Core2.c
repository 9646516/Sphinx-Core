#include <fcntl.h>
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

long long timelimit;
__pid_t pid;
int Exceeded_wall_clock_time = 0;
const int judge_user = 6666;

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
/*****************************************************************************
 * Author: 9646516
 * Used for Interactive Problem
 *****************************************************************************/
int main(int argc, char *argv[]) {
    if (argc != 9)
        errExit("Arguments number should be 8");
    timelimit = atoll(argv[1]);
    long long memorylimit = atoll(argv[2]);
    long long outputlimit = atoll(argv[3]);
    long long stacklimit = atoll(argv[4]);
    char *input_sourcefile = argv[5];
    char *output_sourcefile = argv[6];
    char *running_arguments = argv[7];
    char *checker_sourcefile = argv[8];
    static char sb[114514];
    sprintf(sb, "%s %s %s", checker_sourcefile, input_sourcefile, output_sourcefile);
    if (freopen("/dev/null", "w", stderr) == NULL)
        errExit("Can not redirect stderr");
    int pipe1[2], pipe2[2];
    if (!pipe(pipe1))
        errExit("Failed to make pipe");
    if (!pipe(pipe2))
        errExit("Failed to make pipe");
    __pid_t pid2;
    pid = fork();
    if (pid > 0) {
        pid2 = fork();
        if (pid2 > 0) { //主程序
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
            else if (status_code == SIGIOT)
                goodExit("Assert Failed", timecost / 1000, result.ru_maxrss);
            else if (status_code == SIGXFSZ)
                goodExit("Output Limit Exceeded", timecost / 1000, result.ru_maxrss);
            else if (result.ru_maxrss * 1024 > memorylimit)
                goodExit("Memory Limit Exceeded", timecost / 1000, memorylimit/1024);
            else if (status_code != 0)
                goodExit("Runtime Error", timecost / 1000, result.ru_maxrss);

            FILE *fp = fopen(output_sourcefile, "r");
            if (fp == NULL) {
                errExit("Checker error");
            } else {
                if (!fgets(sb, 1024, fp)) {
                    errExit("Can not get ans");
                }
                goodExit(strncmp(sb, "Accepted", 8) == 0 ? "Accepted" : "Wrong Answer", timecost / 1000, result.ru_maxrss);
            }
        } else if (pid2 == 0) { //测评鸡
            dup2(pipe1[0], STDIN_FILENO);
            dup2(pipe2[1], STDOUT_FILENO);
            execl("/bin/sh", "sh", "-c", sb, (char *)0);
        }
    } else if (pid == 0) { //被测程序
        dup2(pipe1[1], STDOUT_FILENO);
        dup2(pipe2[0], STDIN_FILENO);
        if (!setuid(judge_user))
            errExit("Can not setuid");
        set_limit(RLIMIT_CPU, (timelimit + 999) / 1000, 1);
        set_limit(RLIMIT_DATA, memorylimit, 0);
        set_limit(RLIMIT_FSIZE, outputlimit, 0);
        set_limit(RLIMIT_STACK, stacklimit, 0);
        execl("/bin/sh", "sh", "-c", running_arguments, (char *)0);
    }
    return 0;
}
