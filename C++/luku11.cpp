#include <iostream>

using namespace std;

void osoitin(int *x)
{
    *x = 5;
}

void viittaus(int &x)
{
    x = 5;
}

void toStack(int x)
{
    int y = 2 * x;
    cout << y << "\n";
}

void toHeap(int x)
{
    int *y = new int;
    *y = x * 2;
    cout << *y << "\n";
    delete y;
}

void heapArray()
{
    int *x = new int[10];
    x[0] = 5;
    cout << x[3] << "\n";
    delete x;

}

int main()
{
    int a = 5;
    cout << &a << "\n";
    int *p;
    p = &a;
    *p = 8;
    cout << a << "\n";

    int x = 2;

    osoitin(&x);
    cout << x << "\n";

    x = 2;
    viittaus(x);
    cout << x << "\n";

    x = 2;

    toStack(x);
    toHeap(x);

    heapArray();
}