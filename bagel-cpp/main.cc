#include <iostream>

struct Node {
    int data;
    Node* next;
};

class LinkedList {
public:
    LinkedList() : head(nullptr) {}
    ~LinkedList() {
      Node *temp;
      while (head) {
        temp = head;
        head = head->next;
        delete temp;
      }
    }

    void append(int data) {
        Node* newNode = new Node{data, nullptr};
        if (!head) {
            head = newNode;
        } else {
            Node* temp = head;
            while (temp->next) {
                temp = temp->next;
            }
            temp->next = newNode;
        }
    }

    void print() const {
        Node* temp = head;
        while (temp) {
            std::cout << temp->data << " -> ";
            temp = temp->next;
        }
        std::cout << "nullptr" << std::endl;
    }


private:
    Node* head;
};

int main() {
    LinkedList list;
    list.append(1);
    list.append(2);
    list.append(3);

    list.print();

    return 0;
}