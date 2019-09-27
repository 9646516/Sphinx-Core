#include "testlib/testlib.h"
#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
    setName("Interactor A+B");
    registerInteraction(argc, argv);
    int n = inf.readInt(), ask = 0;
    cerr << "im jury " << n << endl;
    while (true) {
        if (ask++ > 100) {
            tout << "Wrong Answer" << endl;
            quitf(_wa, "%d queries processed", ask);
        }
        int ans = ouf.readInt();
        cerr << "im jury " << ans << ' ' << ask << endl;
        if (ans == n) {
            cout << 1 << endl;
            tout << "Accepted" << endl;
            quitf(_ok, "%d queries processed", ask);
            break;
        } else if (ans > n) {
            cout << 2 << endl;
        } else {
            cout << 0 << endl;
        }
    }
    tout << "114514" << endl;
    return 0;
}
