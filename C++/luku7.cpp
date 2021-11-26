#include <iostream>

using namespace std;

int t[5];
int k[5] = {1};


void printTaulu(int t[]){
    for (int i = 0; i < sizeof(t); i++)
    {
        cout << t[i] << " : ";
    }
    cout << t[sizeof(t)] << "\n";
}

int main(){
    printTaulu(t);
    printTaulu(k);

    t[0] = 5;
    t[1] = 2;
    t[2] = t[0]+t[1];
    t[4] = 178;
    
    printTaulu(t);
}