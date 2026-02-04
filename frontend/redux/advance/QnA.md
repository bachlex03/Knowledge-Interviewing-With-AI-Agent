# Redux Advanced Q&A

## Q1: What is Redux Middleware? - **HIGH**
en: Redux Middleware provides a third-party extension point between dispatching an action and the moment it reaches the reducer. It is commonly used for logging, crash reporting, performing asynchronous tasks (like API calls), and more.
vi: Redux Middleware cung cấp một điểm mở rộng của bên thứ ba giữa việc gửi (dispatch) một action và thời điểm nó đến được reducer. Nó thường được sử dụng để ghi log, báo cáo lỗi, thực hiện các tác vụ bất đồng bộ (như gọi API), v.v.

---

## Q2: Explain Redux Thunk and how it works. - **HIGH**
en: Redux Thunk is a middleware that allows you to write action creators that return a function instead of an action. The thunk can be used to delay the dispatch of an action, or to dispatch only if a certain condition is met. This is ideal for handling asynchronous logic.
vi: Redux Thunk là một middleware cho phép bạn viết các action creators trả về một hàm thay vì một action. Thunk có thể được sử dụng để trì hoãn việc gửi một action, hoặc chỉ gửi nếu một điều kiện nhất định được đáp ứng. Điều này lý tưởng để xử lý các logic bất đồng bộ.

#### Example / Ví dụ:
```javascript
const fetchUser = (userId) => {
  return async (dispatch, getState) => {
    dispatch({ type: 'FETCH_USER_REQUEST' });
    try {
      const response = await api.getUser(userId);
      dispatch({ type: 'FETCH_USER_SUCCESS', payload: response.data });
    } catch (error) {
      dispatch({ type: 'FETCH_USER_FAILURE', error });
    }
  };
};
```

---

## Q3: What is Redux Saga and how does it differ from Thunk? - **HIGH**
en: Redux Saga is a middleware that uses ES6 Generators to make asynchronous flows (side effects) easier to read, write, and test. Unlike Thunk, where async logic is inside action creators, Saga listens for actions and runs separate generator functions. Sagas are more powerful for complex flows (e.g., race conditions, cancellations).
vi: Redux Saga là một middleware sử dụng ES6 Generators để giúp các luồng bất đồng bộ (side effects) dễ đọc, dễ viết và dễ kiểm tra hơn. Khác với Thunk, nơi logic bất đồng bộ nằm bên trong các action creators, Saga lắng nghe các actions và chạy các hàm generator riêng biệt. Sagas mạnh mẽ hơn cho các luồng phức tạp (ví dụ: race conditions, hủy bỏ tác vụ).

---

## Q4: Explain the concept of "Immutability" in Redux. - **HIGH**
en: Immutability means that once an object or array is created, it cannot be changed. In Redux, you never mutate the state directly. Instead, you create a copy of the existing state and update the copy. This allows React to detect changes efficiently (via reference equality) and enables features like time-travel debugging.
vi: Tính bất biến (Immutability) nghĩa là một khi đối tượng hoặc mảng được tạo ra, nó không thể bị thay đổi. Trong Redux, bạn không bao giờ thay đổi trực tiếp state. Thay vào đó, bạn tạo một bản sao của state hiện tại và cập nhật trên bản sao đó. Điều này cho phép React phát hiện các thay đổi một cách hiệu quả (thông qua so sánh tham chiếu) và cho phép các tính năng như gỡ lỗi du hành thời gian.

---

## Q5: How does selectors (Reselect) help in performance optimization? - **HIGH**
en: Selectors are functions that extract pieces of state. The Reselect library provides a way to create memoized selectors. They only recalculate their result if the input state slices have changed. This prevents unnecessary re-renders of components that depend on derived data.
vi: Selectors là các hàm trích xuất các phần dữ liệu từ state. Thư viện Reselect cung cấp một cách để tạo ra các memoized selectors (các selector có ghi nhớ). Chúng chỉ tính toán lại kết quả nếu các phần state đầu vào thay đổi. Điều này ngăn việc render lại không cần thiết của các thành phần phụ thuộc vào dữ liệu được tính toán.

---

## Q6: What is Redux Toolkit (RTK) and why is it recommended? - **MEDIUM**
en: Redux Toolkit is the official, opinionated, batteries-included toolset for efficient Redux development. It simplifies common tasks like store setup, creating reducers with Immer (for easier immutable updates), and built-in Thunk support. It reduces boilerplate code significantly.
vi: Redux Toolkit là bộ công cụ chính thức, có định hướng và đầy đủ tính năng để phát triển Redux hiệu quả. Nó đơn giản hóa các tác vụ phổ biến như thiết lập store, tạo reducers với Immer (để cập nhật bất biến dễ dàng hơn) và tích hợp sẵn hỗ trợ Thunk. Nó giúp giảm thiểu đáng kể mã lặp (boilerplate).

---

## Q7: What is State Normalization and why should you do it? - **HIGH**
en: Normalization means structuring your state so that there is no duplication of data, and every item is stored in a lookup table (object) by ID. This makes it easier to update a single item globally and prevents data inconsistency issues common with nested structures.
vi: Chuẩn hóa (Normalization) nghĩa là cấu trúc lại state của bạn sao cho không có sự trùng lặp dữ liệu, và mỗi mục được lưu trữ trong một bảng tra cứu (object) theo ID. Điều này giúp cập nhật một mục duy nhất trên toàn cục dễ dàng hơn và ngăn chặn các vấn đề không nhất quán dữ liệu thường gặp với các cấu trúc lồng nhau.

---

## Q8: Explain Flux vs. Redux. - **MEDIUM**
en: Flux is a pattern, while Redux is a specific implementation of it. Redux simplifies Flux by having only one store (Flux can have many), no Dispatcher (the store handles dispatches), and uses pure functions (reducers) instead of stores that handle their own logic.
vi: Flux là một mẫu thiết kế (pattern), trong khi Redux là một bản triển khai cụ thể của nó. Redux đơn giản hóa Flux bằng cách chỉ có một store duy nhất (Flux có thể có nhiều), không có Dispatcher (store tự xử lý việc dispatch) và sử dụng các hàm thuần túy (reducers) thay vì các stores tự xử lý logic của chính chúng.

---

## Q9: How to handle persistent state in Redux? - **MEDIUM**
en: Persistent state is handled using libraries like `redux-persist`, which automatically saves the Redux store to local storage or other storage engines and rehydrates (restores) it when the application starts.
vi: Trạng thái bền vững (Persistent state) được xử lý bằng các thư viện như `redux-persist`, tự động lưu store Redux vào local storage hoặc các công cụ lưu trữ khác và "tái hợp nước" (restores) nó khi ứng dụng khởi động.

---

## Q10: What is Redux DevTools and how does it help? - **LOW**
en: Redux DevTools is a powerful browser extension that allows you to inspect every state change, see the action that triggered it, and "travel back in time" by toggling actions on and off. It is indispensable for debugging complex state logic.
vi: Redux DevTools là một tiện ích mở rộng mạnh mẽ của trình duyệt cho phép bạn kiểm tra mọi thay đổi trạng thái, xem action đã kích hoạt nó và "du hành ngược thời gian" bằng cách bật/tắt các actions. Nó là công cụ không thể thiếu để gỡ lỗi các logic trạng thái phức tạp.
