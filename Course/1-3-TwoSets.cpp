#include <iostream>
#include <vector>

using namespace std;

int main()
{
    long n;
    cin >> n;
    long sum = n * (n + 1) / 2;

    if (sum % 2 == 1)
    {
        cout << "NO";
        return NULL;
    }
    long divdSum = sum / 2;
    vector<int> first, last;
    for (int i = n; i >= 1; i--)
    {   
        if(i <= divdSum){
            first.push_back(i);
            divdSum -= i;
            continue;
        }
        last.push_back(i);

    }

    cout << "YES"
         << "\n";
    cout << first.size() << "\n";

    for (long i = 0; i < first.size(); i++)
    {
        cout << first[i] << " ";
    }
    cout << "\n" << last.size() << "\n";

    for (long i = 0; i < last.size(); i++)
    {
        cout << last[i] << " ";
    }
}