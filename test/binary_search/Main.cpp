#include <bits/stdc++.h>

using namespace std;

int main(int argc, char *argv[]) {
    int L = 1, R = 1e9, res;
    while (L <= R) {
        int mid = (L + R) / 2;
        cout << mid << endl;
        cin >> res;
        cerr << "im solution " << res << endl;
        if (res == 1)
            return 0;
        else if (res == 2) {
            R = mid - 1;
        } else {
            L = mid + 1;
        }
    }
    return 0;
}
