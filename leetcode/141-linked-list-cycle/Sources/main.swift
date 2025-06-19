// The Swift Programming Language
// https://docs.swift.org/swift-book

// Definition for singly-linked list.
public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init(_ val: Int) {
        self.val = val
        self.next = nil
    }
}

class Solution {
    func hasCycle(_ head: ListNode?) -> Bool {
        guard let head else { return false }
        
        var slow: ListNode? = head
        var fast: ListNode? = head
        
        repeat {
            slow = slow?.next
            fast = fast?.next?.next
            
            if let slow, let fast {
                if slow.val == fast.val {
                    return true
                }
            }
        } while fast != nil
        
        return false
    }
}
