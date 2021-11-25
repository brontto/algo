#include <iostream>

using namespace std;

int main(){
    int a;
    a = 5;

    int b = 2, c = 4;

    cout << a << " : " << b << "\n" << c << "\n";

    a = b + c;

    cout << a << "\n";

    a++;

    cout << a << " : " << b << "\n";

    swap(a, b);

    cout << a << " : " << b << "\n";

    const int p = 5;

    cout << p << "\n";


}
