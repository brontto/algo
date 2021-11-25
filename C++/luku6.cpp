#include <iostream>

using namespace std;



void rivi (int n){
    for (int i = 1; i <= n; i++)
    {
        cout << "*";
    }
    cout << "\n";

}

int pienin(int a, int b){
    if(a < b) return a;
    else return b;
}

int main(){
    rivi(5);
    rivi(8);
    cout << pienin(5,3) << "\n";

}