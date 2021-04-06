#include<stdio.h>
#include<math.h>
int main(void)
{
    int count=0;float cash;
     do{
     printf("Change Owed : ");
    scanf("%f",&cash);}while(cash<=0);
    cash=round(cash*100);
   
        while(cash>=25)
        {
            cash=cash-25;
            count++;
        }
        while(cash>=10)
        {
            cash=cash-10;

            count++;
        }
        while(cash>=5)
        {
            cash=cash-5;

            count++;
        }
        while(cash>=1)
        {
            cash=cash-1;

            count++;
        }

    printf("%d",count);
}