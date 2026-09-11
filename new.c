#include <unistd.h>



void ft_putchar(char c)
{
    write(1, &c, 1);
}


int main()
{
    int x = 0;
    for (int i = 0; i < 9; i++)
    {
        ft_putchar(i + '0');
    }
}