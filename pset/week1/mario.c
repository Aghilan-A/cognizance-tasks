#include<stdio.h>
int main(void)
{
    int row,n=2;
    do{
    printf("Height : ");
    scanf("%d",&row);}while(row<0 || row>23);

if(row>=0 && row<=23)
{
for(int i=0;i<row;i++)
       {
        for(int j=0;j<=row-n;j++)
            {
                printf(" ");
            }
        for(int k=row-n; k<row;k++)
            {
                printf("#");
            }
            n++;
            printf("\n");
        }
    }

}