#include <iostream>
#include <vector>

using namespace std;

int main (){
    int n;
    cin >> n;
    
    vector<int> t;
    int x; 
    for (int i = 0; i < n; i++)
    {
        cin >> x;
        t.push_back(x);
    }
    
    long r = 0;
    
    for (int i = 1; i < n; i++)
    {
        while (t[i-1] > t[i]){
            t[i]++;
            r++;
        }
    }
    

    cout << r;
    

}