public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init() { self.val = 0; self.next = nil; }
    public init(_ val: Int) { self.val = val; self.next = nil; }
    public init(_ val: Int, _ next: ListNode?) { self.val = val; self.next = next; }
}

class Solution {
    func addTwoNumbers(_ l1: ListNode?, _ l2: ListNode?) -> ListNode? {
//        This problem is essentially column addition, so we work it out like we would there.
        
//        We make a copy of the linked lists so that we can easily iterate through them
        var l1copy = l1
        var l2copy = l2
        
        var finalAns: ListNode?
        
//        We need to track a carry value between iterations so that for stuff like 4+6 we can add 1 to the next digit
        var carry = 0
        
//        Linked lists need do-while loops (or repeat-while if you're Swift) so that the last value is not skipped
        repeat {
//            We set null values to 0 so that lists of a different length can be calculated (the shorter list effectively has an infinite amount of leading 0s
            let num1 = l1copy?.val ?? 0
            let num2 = l2copy?.val ?? 0
            
            var ans = num1 + num2
            ans += carry
            carry = 0
            
//            If the answer is greater than 9, set the carry value and set the answer to the least significant digit
            if ans > 9 {
                carry = 1
                ans = ans % 10
            }
            
//            If we're on the first number, initialise a linked list with the result
            if finalAns == nil {
                finalAns = ListNode(ans)
            } else {
//                Otherwise, iterate through the list until we find the end node
                var node = finalAns
                
                while node!.next != nil {
                    node = node!.next
                }
                
//                Once we've found the end node, append the digit
                node!.next = ListNode(ans)
            }
            
//            Shift along the lists
            l1copy = l1copy?.next
            l2copy = l2copy?.next
            
//            We check if the carry is 0 to ensure that we've always applied it, even if it just gets put at the end (9999999 + 9999 needs this)
        } while l1copy != nil || l2copy != nil || carry != 0
        
        return finalAns
    }
}

