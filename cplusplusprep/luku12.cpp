#include <iostream>

using namespace std;

struct piste {
    int x, y;
};

int main(){
    piste a = {1, 3};
    
    cout << a.x << " : " << a.y << "\n";

    a.x = 2;
    cout << a.x << " : " << a.y << "\n";

}