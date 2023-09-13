#include <stdio.h>
void HelloPrint(){
	const char* text = "hello world!";
	printf("%s\n",text);
}
int main(int argc,char** argv){
	HelloPrint();
}
