```mermaid
flowchart
n1(Add text)
n2(Sub text)
n3(Insert)
n4(Node)
n5(Node)
t1[Original text]
t2[Pasted text]
Start --> n1
Start --> n2
	n1 --> n5
	n2 --> n3
	n2 --> n4
	n3 --> t1
	n5 --> t1
	Start --> t2

```


