import fileinput as fi


class Node:
    def __init__(self, address, data):
        self.address = address
        self.data = data
        self.height = -1
        self.left = None
        self.right = None

    def update(self, data):
        self.data = data


class Tree:
    def insert(self, root, address, data):
        # Base case
        if root == None:
            return Node(address, data)
        elif address < root.address:
            root.left = self.insert(root.left, address, data)
        elif address > root.address:
            root.right = self.insert(root.right, address, data)
        # In the case of duplicate node address, overwrite
        else:
            root.data = data
            return root

        # Update height
        root.height = 1 + max(self.get_height(root.left),
                              self.get_height(root.right))

        # Get balance factor
        balance_factor = self.get_balance(root)

        # Left-heavy root
        if balance_factor <= -2:
            # Left-heavy child (LL)
            if self.get_balance(root.left) < 0:
                return self.rotate_right(root)
            # Right-heavy child (LR)
            else:
                root.left = self.rotate_left(root.left)
                return self.rotate_right(root)
        # Right-heavy root
        elif balance_factor >= 2:
            # Right-heavy child (RR)
            if self.get_balance(root.right) > 0:
                return self.rotate_left(root)
            # Left-heavy child (RL)
            else:
                root.right = self.rotate_right(root.right)
                return self.rotate_left(root)

        return root

    def get_height(self, root):
        if root == None:
            return -1
        return root.height

    def get_balance(self, root):
        if root == None:
            return 0
        return self.get_height(root.right) - self.get_height(root.left)

    def rotate_left(self, old_root):
        new_root = old_root.right

        # Rotate
        orphan = new_root.left
        new_root.left = old_root
        old_root.right = orphan

        # Update heights
        old_root.height = 1 + \
            max(self.get_height(old_root.left), self.get_height(old_root.right))
        new_root.height = 1 + \
            max(self.get_height(new_root.left), self.get_height(new_root.right))

        return new_root

    def rotate_right(self, old_root):
        new_root = old_root.left

        # Rotate
        orphan = new_root.right
        new_root.right = old_root
        old_root.left = orphan

        # Update heights
        old_root.height = 1 + \
            max(self.get_height(old_root.left), self.get_height(old_root.right))
        new_root.height = 1 + \
            max(self.get_height(new_root.left), self.get_height(new_root.right))

        return new_root


def parse_data(mask, data):
    data = int(data)
    mask = mask[1:len(mask)]
    for i, bit in enumerate(mask):
        if bit == '1':
            data |= (1 << i)
        elif bit == '0':
            data &= ~(1 << i)

    return data


def tree_sum(root):
    # Base case
    if root == None:
        return 0

    return root.data + tree_sum(root.left) + tree_sum(root.right)


mask = ""
value = 0
tree = Tree()
root = None

for line in fi.input(files='input.txt'):
    line = line.split(" = ")

    # Parse between mask updates or memory sets
    if line[0] == "mask":
        mask = line[1][::-1]
        continue
    else:
        address = int(line[0][4:len(line[0]) - 1])
        data = parse_data(mask, line[1])
        root = tree.insert(root, address, data)

print(tree_sum(root))