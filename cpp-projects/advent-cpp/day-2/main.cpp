#include <iostream>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

//checker, return if valid
bool check_pass(string ip)
{
    int iplen = ip.length();
    int index_dash, index_char;

    for (int i = 0; i < iplen; i++)
    {
        if (ip[i] == '-')
        {
            index_dash = i;
        }
        if (ip[i] == ':')
        {
            index_char = i - 1;
            break;
        }
    }

    int limin = stoi(ip.substr(0, index_dash));
    int limax = stoi(ip.substr(index_dash + 1, index_char - index_dash - 2));
    char key = ip[index_char];
    string password = ip.substr(index_char + 3, ip.length() - index_char - 2);

    int passlen = password.length();
    int count = 0;

    for (int i = 0; i < passlen; i++)
    {
        if (password[i] == key)
        {
            count++;
        }
    }

    return count >= limin && count <= limax;
}

int main(void)
{
    string buffer;
    int counter = 0;
    ifstream ifp;
    ifp.open("input.txt", ios::in);

    while (!ifp.eof())
    {
        getline(ifp, buffer);

        if (check_pass(buffer)) {
            counter++;
        }
    }
    cout << counter <<" are valid.";
}