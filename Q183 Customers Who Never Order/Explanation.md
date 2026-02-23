# 🧩 LeetCode Problem: Customers Who Never Order

- **Problem Link**: <https://leetcode.com/problems/customers-who-never-order/>
- **Goal**: Find all customers who have never placed an order.

---

## 🧠 Algorithm Explanation

We use the **NOT EXISTS** approach to determine which customers have no related records in the `Orders` table.

The logic is based on the relationship:

- `Customers.id` is the primary key.
- `Orders.customerId` references `Customers.id`.

For each customer, we check whether there exists any row in `Orders` where:

Orders.customerId = Customers.id

If such a row exists → the customer has placed an order → exclude them.  
If no such row exists → the customer never ordered → include them.

`NOT EXISTS` is chosen because:

- It directly expresses the required condition (“no matching record exists”).
- It short-circuits evaluation once a match is found.
- It avoids constructing a full join result.
- It performs efficiently when indexed properly.

---

### 🪜 Steps

1. Iterate through each customer in the `Customers` table.
2. For each customer, check if a matching order exists in the `Orders` table.
3. Use `NOT EXISTS` to filter customers where no matching order is found.
4. Return the names of those customers.

---

## ✅ Constraints

- `Customers.id` is unique.
- `Orders.customerId` is a foreign key.
- Each order belongs to one customer.
- Output order does not matter.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n + m)   |
| Space Complexity  | O(1)       |

Where:

- `n` = number of customers  
- `m` = number of orders  

With proper indexing on `Orders.customerId`, the query runs efficiently.
