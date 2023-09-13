#include <iostream>
#include <string>
void HelloPrint(){
	const std::string text("hello world!");
	std::cout << text << std::endl;
}
int main(int argc,char** argv){
	HelloPrint();
}
