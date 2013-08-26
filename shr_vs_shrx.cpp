#define XBYAK_NO_OP_NAMES
#include <xbyak/xbyak_util.h>
#include <cybozu/benchmark.hpp>

const int N = 10000000;

int (*f1)(); // shrx
int (*f2)(); // shr
int (*f3)(); // rorx 1
int (*f4)(); // ror 1
int (*f5)();
int (*f6)();

struct Code : Xbyak::CodeGenerator {
	Code()
	{
		f1 = getCurr<int (*)()>();
		gen1();
		align(16);
		f2 = getCurr<int (*)()>();
		gen2();
		align(16);
		f3 = getCurr<int (*)()>();
		gen3(1);
		align(16);
		f4 = getCurr<int (*)()>();
		gen4(1);

		align(16);
		f5 = getCurr<int (*)()>();
		gen3(5);
		align(16);
		f6 = getCurr<int (*)()>();
		gen4(5);
	}
	void gen1()
	{
		mov(ecx, N);
		xor_(eax, eax);
		mov(edx, 1);
	L("@@");
		shrx(r8, rcx, rdx);
		add(rax, r8);
		dec(ecx);
		jnz("@b");
		ret();
	}
	void gen2()
	{
		mov(ecx, N);
		xor_(eax, eax);
	L("@@");
		mov(rdx, rcx);
		shr(rdx, 1);
		add(rax, edx);
		dec(ecx);
		jnz("@b");
		ret();
	}
	void gen3(int shift)
	{
		mov(ecx, N);
		xor_(eax, eax);
	L("@@");
		rorx(r8, rcx, shift);
		add(rax, r8);
		sub(ecx, 1);
		jnz("@b");
		ret();
	}
	void gen4(int shift)
	{
		mov(ecx, N);
		xor_(eax, eax);
	L("@@");
		mov(r8, rcx);
		ror(r8, shift);
		add(rax, r8);
		sub(ecx, 1);
		jnz("@b");
		ret();
	}
};

int main()
{
	Code c;
	printf("%x\n", f1());
	printf("%x\n", f2());
	CYBOZU_BENCH("f1", f1);
	CYBOZU_BENCH("r2", f2);

	printf("%x\n", f3());
	printf("%x\n", f4());
	CYBOZU_BENCH("f3", f3);
	CYBOZU_BENCH("r4", f4);

	printf("%x\n", f5());
	printf("%x\n", f6());
	CYBOZU_BENCH("f3", f3);
	CYBOZU_BENCH("r4", f4);
}

