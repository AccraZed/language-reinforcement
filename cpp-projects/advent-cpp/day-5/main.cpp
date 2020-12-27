#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int getId(string ip)
{
    int ip_len = ip.length();
    int row_start = 0;
    int col_start = 0;
    int n = 64;
    int m = 512;
    for (int i = 0; i < ip_len; i++)
    {
        if (ip.substr(i, 1) == "B")
        {
            row_start += n;
        }
        n /= 2;

        if (ip.substr(i, 1) == "R")
        {
            col_start += m;
        }
        m /= 2;
    }
    return (row_start * 8) + col_start;
}

int main(void)
{
    ifstream ifp;
    string buffer;
    ifp.open("input.txt", ios::in);
    int high = 0;
    int current = 0;

    while (!ifp.eof())
    {
        getline(ifp, buffer);
        current = getId(buffer);
        if (current >= high)
        {
            high = current;
        }
    }

    cout << "The highest seat ID is " << high << ".";
}