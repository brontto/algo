#include <iostream>
#include <vector>

using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<int> vec, pos (n);
    for (int i = 0; i < n; i++)
    {
        int x;
        cin >> x;
        vec.push_back(x);
        pos[vec[i]] = i;
    }

    int r = 1;
    for (int i = 1; i <= n; i++)
    {
        if (pos[i - 1] > pos[i])
        {
            r++;
        }
    }

    cout << r << endl;
}