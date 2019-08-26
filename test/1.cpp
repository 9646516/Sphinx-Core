#include <bits/stdc++.h>
using namespace std;
int main() {
    for (char i : "Hello World") {
        cout << i;
    }
    cout << endl;
    for (auto i : vector<char>{
             'h',
             'e',
             'l',
             'l',
             'o',
             ' ',
             'W',
             'o',
             'r',
             'l',
             'd',
         }) {
        cout << i;
    }
    cout << endl;
    return 0;
}
