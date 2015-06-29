mod Stack {

pub struct Stack;

impl Stack {
	pub fn new() -> Stack {
		Stack
	}
}
}

/*
type Stack struct {
	Items   *[]ins.Node
	Parents []*[]ins.Node
}

func (stack *Stack) Pop() {
	if len(stack.Parents) == 0 {
		items := make([]ins.Node, 0)
		stack.Items = &items
		return
	}

	stack.Items = stack.Parents[len(stack.Parents)-1]
	stack.Parents = stack.Parents[:len(stack.Parents)-1]
}

func (stack *Stack) Push() {
	stack.Parents = append(stack.Parents, stack.Items)

	items := make([]ins.Node, 0)
	stack.Items = &items
}

func (stack *Stack) Add(node ins.Node) {
	items := *stack.Items
	items = append(items, node)

	stack.Items = &items
}

func (stack *Stack) Reset() {
	stack.Empty()
	stack.Parents = make([]*[]ins.Node, 0)
}

func (stack *Stack) Empty() {
	items := make([]ins.Node, 0)
	stack.Items = &items
}
*/