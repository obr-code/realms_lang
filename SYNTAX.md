Mutability:
C: Constant
M: Mutable
S: Static

Locality:
L: Local
G: Global

Object:
struct


Ex:

struct Human {
	name: Str,
}
impl Human {
	GC new: Fn = |name: Str| -> Human {
		Human { name }
	}
}

LC name: Str = "Olivier";
LC human: Human = Human(name);
	