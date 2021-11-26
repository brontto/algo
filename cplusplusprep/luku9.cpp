#include <iostream>

using namespace std;

using ll = long long;

int main()
{
    ll value = 4;
    cout << value << "\n";

    cout << 1 / 2 << "\n";
    cout << (double)1 / 2 << "\n";
    cout << 1.0 / 2 << "\n";
    cout << 1 / 2.0 << "\n";
    cout << 1.0 / 2.0 << "\n";

    int x = 2147483647;
    x++;
    cout << x << "\n"; // -2147483648

    double k = 1e308;
    k *= 2;
    cout << k << "\n";

    double a = 0.3 * 3 + 0.1;
    double b = 1;
    cout << a << " " << b << "\n";
    if (a == b)
        cout << "luvut ovat samat\n";
    if (a < b)
        cout << "a on pienempi\n";
    if (a > b)
        cout << "a on suurempi\n";
}