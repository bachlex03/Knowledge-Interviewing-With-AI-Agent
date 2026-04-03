# Technical Foundations: Big O Notation / Kiến thức nền tảng: Ký pháp Big O

en: This document covers the fundamental concepts of Big O notation, which is essential for analyzing algorithm efficiency in data structures and algorithms.
vi: Tài liệu này trình bày các khái niệm cơ bản về ký pháp Big O, một yếu tố thiết yếu để phân tích hiệu quả của thuật toán trong cấu trúc dữ liệu và giải thuật.

---

## 1. What is Big O Notation? / Ký pháp Big O là gì?

en: Big O notation is a mathematical notation used in computer science to describe the performance or complexity of an algorithm. Specifically, it describes the **worst-case scenario** and how the execution time or space requirements grow as the input size increases.
vi: Ký pháp Big O là một ký hiệu toán học được sử dụng trong khoa học máy tính để mô tả hiệu suất hoặc độ phức tạp của một thuật toán. Cụ thể, nó mô tả **trường hợp xấu nhất** và cách thời gian thực thi hoặc yêu cầu không gian lưu trữ tăng lên khi kích thước đầu vào tăng.

---

## 2. Why do we need Big O? / Tại sao chúng ta cần Big O?

en: Without Big O, measuring performance would depend on hardware (CPU speed, memory). Big O provides a consistent way to compare algorithms by focusing on how they scale with the input size ($n$).
vi: Nếu không có Big O, việc đo lường hiệu suất sẽ phụ thuộc vào phần cứng (tốc độ CPU, bộ nhớ). Big O cung cấp một cách nhất quán để so sánh các thuật toán bằng cách tập trung vào cách chúng mở rộng quy mô theo kích thước đầu vào ($n$).

---

## 3. Common Big O Notations / Các ký pháp Big O phổ biến

| Notation | Name / Tên gọi | Description / Mô tả |
| :--- | :--- | :--- |
| **$O(1)$** | **Constant Time** / Thời gian hằng số | en: Execution time is independent of input size.<br>vi: Thời gian thực thi không phụ thuộc vào kích thước đầu vào. |
| **$O(\log n)$** | **Logarithmic Time** / Thời gian logarit | en: Time increases logarithmically (e.g., Binary Search).<br>vi: Thời gian tăng theo hàm logarit (ví dụ: Tìm kiếm nhị phân). |
| **$O(n)$** | **Linear Time** / Thời gian tuyến tính | en: Time increases proportionally with input size.<br>vi: Thời gian tăng tỷ lệ thuận với kích thước đầu vào. |
| **$O(n \log n)$** | **Linearithmic Time** / Thời gian tuyến tính - logarit | en: Common in efficient sorting algorithms like Merge Sort.<br>vi: Phổ biến trong các thuật toán sắp xếp hiệu quả như Merge Sort. |
| **$O(n^2)$** | **Quadratic Time** / Thời gian bậc hai | en: Time increases with the square of the input size (e.g., nested loops).<br>vi: Thời gian tăng theo bình phương kích thước đầu vào (ví dụ: vòng lặp lồng nhau). |
| **$O(2^n)$** | **Exponential Time** / Thời gian mũ | en: Recursive algorithms with multiple branches.<br>vi: Các thuật toán đệ quy có nhiều nhánh rẽ. |
| **$O(n!)$** | **Factorial Time** / Thời gian giai thừa | en: Extremely slow, often seen in permutation problems.<br>vi: Cực kỳ chậm, thường thấy trong các bài toán hoán vị. |

---

## 4. Code Examples / Ví dụ Mã nguồn

### O(1) - Constant Time / Thời gian hằng số
en: Accessing an element in an array by index.
vi: Truy cập một phần tử trong mảng thông qua chỉ số.
```python
def get_first_element(arr):
    return arr[0]
```

```cpp
int get_first_element(const std::vector<int>& arr) {
    return arr[0];
}
```

```csharp
public int GetFirstElement(int[] arr) {
    return arr[0];
}
```

```rust
fn get_first_element(arr: &[i32]) -> i32 {
    arr[0]
}
```

### O(log n) - Logarithmic Time / Thời gian logarit
en: Binary search on a sorted array.
vi: Tìm kiếm nhị phân trên một mảng đã sắp xếp.
```python
def binary_search(arr, target):
    low, high = 0, len(arr) - 1
    while low <= high:
        mid = (low + high) // 2
        if arr[mid] == target: return mid
        elif arr[mid] < target: low = mid + 1
        else: high = mid - 1
    return -1
```

```cpp
int binary_search(const std::vector<int>& arr, int target) {
    int low = 0, high = arr.size() - 1;
    while (low <= high) {
        int mid = low + (high - low) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) low = mid + 1;
        else high = mid - 1;
    }
    return -1;
}
```

```csharp
public int BinarySearch(int[] arr, int target) {
    int low = 0, high = arr.Length - 1;
    while (low <= high) {
        int mid = low + (high - low) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) low = mid + 1;
        else high = mid - 1;
    }
    return -1;
}
```

```rust
fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut low = 0isize;
    let mut high = (arr.len() as isize) - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid as usize] == target { return mid as i32; }
        if arr[mid as usize] < target { low = mid + 1; }
        else { high = mid - 1; }
    }
    -1
}
```

### O(n) - Linear Time / Thời gian tuyến tính
en: Finding an element in an unsorted array.
vi: Tìm kiếm một phần tử trong mảng chưa được sắp xếp.
```python
def find_element(arr, target):
    for i in range(len(arr)):
        if arr[i] == target: return i
    return -1
```

```cpp
int find_element(const std::vector<int>& arr, int target) {
    for (int i = 0; i < arr.size(); ++i) {
        if (arr[i] == target) return i;
    }
    return -1;
}
```

```csharp
public int FindElement(int[] arr, int target) {
    for (int i = 0; i < arr.Length; i++) {
        if (arr[i] == target) return i;
    }
    return -1;
}
```

```rust
fn find_element(arr: &[i32], target: i32) -> i32 {
    for (i, &val) in arr.iter().enumerate() {
        if val == target { return i as i32; }
    }
    -1
}
```

### O(n log n) - Linearithmic Time / Thời gian tuyến tính - logarit
en: Sorting an array using Merge Sort.
vi: Sắp xếp mảng bằng thuật toán Merge Sort.
```python
def merge_sort(arr):
    if len(arr) <= 1: return arr
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    return merge(left, right)
```

```cpp
void merge_sort(std::vector<int>& arr, int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;
        merge_sort(arr, left, mid);
        merge_sort(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}
```

```csharp
public void MergeSort(int[] arr, int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;
        MergeSort(arr, left, mid);
        MergeSort(arr, mid + 1, right);
        Merge(arr, left, mid, right);
    }
}
```

```rust
fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid > 0 {
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        let mut combined = Vec::with_capacity(arr.len());
        // Merge logic...
    }
}
```

### O(n^2) - Quadratic Time / Thời gian bậc hai
en: Bubble Sort with nested loops.
vi: Sắp xếp nổi bọt (Bubble Sort) với các vòng lặp lồng nhau.
```python
def bubble_sort(arr):
    n = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr
```

```cpp
void bubble_sort(std::vector<int>& arr) {
    int n = arr.size();
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n - i - 1; ++j)
            if (arr[j] > arr[j + 1])
                std::swap(arr[j], arr[j + 1]);
}
```

```csharp
public void BubbleSort(int[] arr) {
    int n = arr.Length;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n - i - 1; j++)
            if (arr[j] > arr[j + 1])
                (arr[j], arr[j + 1]) = (arr[j + 1], arr[j]);
}
```

```rust
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
```

### O(2^n) - Exponential Time / Thời gian mũ
en: Naive recursive calculation of Fibonacci.
vi: Tính toán Fibonacci bằng đệ quy cơ bản.
```python
def fibonacci(n):
    if n <= 1: return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

```cpp
int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

```csharp
public int Fibonacci(int n) {
    if (n <= 1) return n;
    return Fibonacci(n - 1) + Fibonacci(n - 2);
}
```

```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 { return n; }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

### O(n!) - Factorial Time / Thời gian giai thừa
en: Generating all permutations of a list.
vi: Tạo tất cả các hoán vị của một danh sách.
```python
def permutations(arr):
    if len(arr) == 0: return [[]]
    result = []
    for i in range(len(arr)):
        first = arr[i]
        remaining = arr[:i] + arr[i+1:]
        for p in permutations(remaining):
            result.append([first] + p)
    return result
```

```cpp
// Using std::next_permutation or manual recursion
void generate_permutations(std::vector<int>& arr) {
    std::sort(arr.begin(), arr.end());
    do {
        // use arr
    } while (std::next_permutation(arr.begin(), arr.end()));
}
```

```csharp
public void Permutations(List<int> arr, int left, int right) {
    if (left == right) // output arr
    else {
        for (int i = left; i <= right; i++) {
            Swap(arr, left, i);
            Permutations(arr, left + 1, right);
            Swap(arr, left, i);
        }
    }
}
```

```rust
fn permutations(arr: Vec<i32>) -> Vec<Vec<i32>> {
    if arr.is_empty() { return vec![vec![]]; }
    // Recursive heap's algorithm or similar...
    Vec::new() 
}
```

---

## 5. How to Calculate Big O / Cách tính Big O

en: To determine the complexity of an algorithm:
1. **Identify the input size** ($n$).
2. **Count the operations**: Focus on how they increase with $n$.
3. **Drop constants**: $O(2n)$ becomes $O(n)$.
4. **Drop lower-order terms**: $O(n^2 + n)$ becomes $O(n^2)$.
5. **Focus on the dominant term**.

vi: Để xác định độ phức tạp của một thuật toán:
1. **Xác định kích thước đầu vào** ($n$).
2. **Đếm các thao tác**: Tập trung vào cách chúng tăng lên theo $n$.
3. **Bỏ qua hằng số**: $O(2n)$ trở thành $O(n)$.
4. **Bỏ qua các hạng tử bậc thấp**: $O(n^2 + n)$ trở thành $O(n^2)$.
5. **Tập trung vào hạng tử chiếm ưu thế nhất**.

---

## 6. Common Pitfalls / Các sai lầm thường gặp

en: 
*   **Counting exact operations**: Big O is about scaling, not exact counts.
*   **Ignoring constants in specific cases**: While Big O drops constants, $O(100n)$ might be slower than $O(n^2)$ for small $n$ in practice.
*   **Confusion between Best, Average, and Worst case**: Always specify which one you are analyzing (usually Worst Case).

vi:
*   **Đếm chính xác số lượng thao tác**: Big O nói về khả năng mở rộng, không phải số lượng chính xác.
*   **Bỏ qua hằng số trong các trường hợp cụ thể**: Mặc dù Big O bỏ qua hằng số, nhưng trên thực tế $O(100n)$ có thể chậm hơn $O(n^2)$ đối với $n$ nhỏ.
*   **Nhầm lẫn giữa trường hợp Tốt nhất, Trung bình và Xấu nhất**: Luôn xác định rõ bạn đang phân tích trường hợp nào (thường là Trường hợp xấu nhất).

---

## 7. Best Practices / Lời khuyên hữu ích

en:
1. Always analyze **Space Complexity** alongside **Time Complexity**.
2. Think about the **constraints** of the input size provided in interview questions.
3. Use Big O as a language to communicate efficiency clearly.

vi:
1. Luôn phân tích **Độ phức tạp không gian (Space Complexity)** song song với **Độ phức tạp thời gian (Time Complexity)**.
2. Suy nghĩ về các **ràng buộc (constraints)** của kích thước đầu vào được đưa ra trong các câu hỏi phỏng vấn.
3. Sử dụng Big O như một ngôn ngữ để trao đổi rõ ràng về hiệu suất.

---

## Conclusion / Kết luận

en: Mastering Big O is the first step toward writing high-quality code. It allows you to anticipate performance issues before they happen in production.
vi: Làm chủ Big O là bước đầu tiên để viết mã chất lượng cao. Nó cho phép bạn dự đoán các vấn đề về hiệu suất trước khi chúng xảy ra trong môi trường thực tế.
