#include<iostream>
using namespace std;
int main()
{
  int x,steps;
  cout<<"Enter value of x : ";
  cin>>x;
  steps=x/5;
  if(x%5!=0)
  {
    steps++;
  }
  cout<<"The minimum number of steps : "<<steps;
  return 0;
}
