# Node.js Foundation Q&A

### Level 1: Remembering

#### Q1: What is Node.js and what is its underlying JavaScript engine?
**Question:**
en: What is Node.js and what is its underlying JavaScript engine?
vi: Node.js là gì và công cụ (engine) JavaScript cốt lõi của nó là gì?

**Answer:**
en: Node.js is an open-source, cross-platform JavaScript runtime environment that executes JavaScript code outside a web browser. It is built on Google Chrome's V8 JavaScript engine.
vi: Node.js là một môi trường thực thi (runtime environment) JavaScript mã nguồn mở, đa nền tảng, thực thi mã JavaScript bên ngoài trình duyệt web. Nó được xây dựng dựa trên V8 JavaScript engine của Google Chrome.

#### Q2: What is the Event Loop in Node.js?
**Question:**
en: What is the Event Loop in Node.js?
vi: Event Loop (vòng lặp sự kiện) trong Node.js là gì?

**Answer:**
en: The Event Loop is the mechanism that allows Node.js to perform non-blocking I/O operations by offloading operations to the system kernel whenever possible, despite JavaScript being single-threaded.
vi: Event Loop là cơ chế cho phép Node.js thực hiện các hoạt động I/O không chặn (non-blocking) bằng cách chuyển giao các hoạt động cho nhân hệ điều hành (system kernel) bất cứ khi nào có thể, mặc dù JavaScript là đơn luồng (single-threaded).

#### Q3: What are Streams in Node.js?
**Question:**
en: What are Streams in Node.js?
vi: Streams (luồng) trong Node.js là gì?

**Answer:**
en: Streams are objects that let you read data from a source or write data to a destination in continuous fashion. There are four types of streams: Readable, Writable, Duplex, and Transform.
vi: Streams là các đối tượng cho phép bạn đọc dữ liệu từ một nguồn hoặc ghi dữ liệu đến một đích một cách liên tục. Có bốn loại streams: Readable (đọc), Writable (ghi), Duplex (hai chiều), và Transform (chuyển đổi).

#### Q4: List the core modules built into Node.js.
**Question:**
en: List some of the core modules built into Node.js.
vi: Liệt kê một số module cốt lõi được tích hợp sẵn trong Node.js.

**Answer:**
en: Some core modules include `http`, `fs` (file system), `path`, `os`, `events`, `crypto`, and `stream`. They provide basic functionalities without needing to install external packages.
vi: Một số module cốt lõi bao gồm `http`, `fs` (hệ thống tệp), `path`, `os`, `events`, `crypto`, và `stream`. Chúng cung cấp các chức năng cơ bản mà không cần cài đặt các gói bên ngoài.

#### Q5: What is the purpose of the `package.json` file?
**Question:**
en: What is the purpose of the `package.json` file?
vi: Mục đích của tệp `package.json` là gì?

**Answer:**
en: The `package.json` file holds metadata relevant to the project, such as the project's name, version, description, and it manages the project's dependencies, scripts, and versions.
vi: Tệp `package.json` chứa siêu dữ liệu (metadata) liên quan đến dự án, chẳng hạn như tên dự án, phiên bản, mô tả, và nó quản lý các thư viện phụ thuộc (dependencies), kịch bản (scripts) và các phiên bản của dự án.

---

### Level 2: Understanding

#### Q1: Explain the difference between synchronous and asynchronous functions in Node.js.
**Question:**
en: Explain the difference between synchronous and asynchronous functions in Node.js.
vi: Giải thích sự khác biệt giữa các hàm đồng bộ (synchronous) và bất đồng bộ (asynchronous) trong Node.js.

**Answer:**
en: Synchronous functions block the execution thread until they complete, preventing subsequent code from running. Asynchronous functions execute in the background and use callbacks, promises, or async/await to notify completion, allowing the main thread to continue without blocking.
vi: Các hàm đồng bộ chặn luồng thực thi cho đến khi chúng hoàn thành, ngăn mã tiếp theo chạy. Các hàm bất đồng bộ thực thi ngầm và sử dụng callbacks, promises hoặc async/await để thông báo khi hoàn thành, cho phép luồng chính tiếp tục mà không bị chặn.

#### Q2: Describe how the Event Loop handles asynchronous callbacks.
**Question:**
en: Describe how the Event Loop handles asynchronous callbacks.
vi: Mô tả cách Event Loop xử lý các callback bất đồng bộ.

**Answer:**
en: When an asynchronous operation (like a timer or network request) completes, its callback is pushed to an event queue. The Event Loop continuously checks if the call stack is empty. If it is, it picks the first callback from the queue and pushes it to the call stack for execution.
vi: Khi một hoạt động bất đồng bộ (như bộ đếm thời gian hoặc yêu cầu mạng) hoàn thành, callback của nó được đẩy vào một hàng đợi sự kiện (event queue). Event Loop liên tục kiểm tra xem call stack có trống hay không. Nếu trống, nó sẽ lấy callback đầu tiên từ hàng đợi và đẩy vào call stack để thực thi.

#### Q3: Discuss the difference between `require()` (CommonJS) and `import` (ES Modules) in Node.js.
**Question:**
en: Discuss the difference between `require()` (CommonJS) and `import` (ES Modules) in Node.js.
vi: Thảo luận về sự khác biệt giữa `require()` (CommonJS) và `import` (ES Modules) trong Node.js.

**Answer:**
en: `require()` is a CommonJS module loading system that is synchronous and dynamic (can be conditionally called). `import` is the ES Module standard, which is asynchronous, statically analyzable at parse time, and allows for tree-shaking.
vi: `require()` là hệ thống tải module của CommonJS, hoạt động đồng bộ và linh hoạt (có thể gọi theo điều kiện). `import` là tiêu chuẩn ES Module, hoạt động bất đồng bộ, có thể phân tích tĩnh tại thời điểm parse (parse time) và cho phép tree-shaking (loại bỏ code không sử dụng).

#### Q4: Summarize the purpose of the `Buffer` class in Node.js.
**Question:**
en: Summarize the purpose of the `Buffer` class in Node.js.
vi: Tóm tắt mục đích của lớp `Buffer` trong Node.js.

**Answer:**
en: The `Buffer` class is used to work with raw binary data directly in memory. It is particularly useful when dealing with streams, reading from files, or handling network protocols where raw binary data needs to be manipulated.
vi: Lớp `Buffer` được sử dụng để làm việc trực tiếp với dữ liệu nhị phân thô trong bộ nhớ. Nó đặc biệt hữu ích khi xử lý với streams, đọc từ tệp hoặc xử lý các giao thức mạng nơi dữ liệu nhị phân thô cần được thao tác.

#### Q5: Explain how Node.js achieves concurrency despite being single-threaded.
**Question:**
en: Explain how Node.js achieves concurrency despite being single-threaded.
vi: Giải thích cách Node.js đạt được tính đồng thời (concurrency) mặc dù chỉ có một luồng (single-threaded).

**Answer:**
en: Node.js achieves concurrency by delegating I/O operations to the operating system using asynchronous APIs. While the OS handles these tasks concurrently in the background, the single Node.js main thread continues executing other JavaScript code. Once OS tasks complete, their callbacks are handled via the Event Loop.
vi: Node.js đạt được tính đồng thời bằng cách ủy thác các hoạt động I/O cho hệ điều hành thông qua các API bất đồng bộ. Trong khi OS xử lý đồng thời các tác vụ này ngầm, luồng chính duy nhất của Node.js tiếp tục thực thi các mã JavaScript khác. Khi các tác vụ của OS hoàn thành, các callback của chúng được xử lý qua Event Loop.

---

### Level 3: Applying

#### Q1: Write a basic HTTP server in Node.js.
**Question:**
en: Write a basic HTTP server in Node.js that responds with "Hello World" to all requests.
vi: Viết một máy chủ HTTP cơ bản trong Node.js phản hồi bằng "Hello World" cho tất cả các yêu cầu.

**Answer:**
en: You can use the built-in `http` module to create a server and listen on a specific port.
vi: Bạn có thể sử dụng module `http` được tích hợp sẵn để tạo máy chủ và lắng nghe trên một cổng cụ thể.
```javascript
const http = require('http');

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Hello World\\n');
});

server.listen(3000, () => {
  console.log('Server running at http://localhost:3000/');
});
```

#### Q2: Demonstrate how to read a file asynchronously using the `fs/promises` module.
**Question:**
en: Demonstrate how to read a file asynchronously using the `fs/promises` module.
vi: Minh họa cách đọc một tệp một cách bất đồng bộ bằng module `fs/promises`.

**Answer:**
en: Use `async/await` with the `readFile` function from `fs/promises` to asynchronously read the file content without blocking the event loop.
vi: Sử dụng `async/await` với hàm `readFile` từ `fs/promises` để đọc nội dung tệp một cách bất đồng bộ mà không làm chặn event loop.
```javascript
const fs = require('fs/promises');

async function readMyFile() {
  try {
    const data = await fs.readFile('data.txt', 'utf8');
    console.log(data);
  } catch (error) {
    console.error('Error reading file:', error);
  }
}

readMyFile();
```

#### Q3: Create a custom event emitter.
**Question:**
en: Use the `events` module to create a custom event emitter and handle a 'userLoggedIn' event.
vi: Sử dụng module `events` để tạo một event emitter tùy chỉnh và xử lý sự kiện 'userLoggedIn'.

**Answer:**
en: Instantiate `EventEmitter`, attach a listener using `.on()`, and trigger the event using `.emit()`.
vi: Khởi tạo `EventEmitter`, gắn một listener bằng cách sử dụng `.on()`, và kích hoạt sự kiện bằng `.emit()`.
```javascript
const EventEmitter = require('events');

class MyEmitter extends EventEmitter {}
const myEmitter = new MyEmitter();

myEmitter.on('userLoggedIn', (username) => {
  console.log(`Welcome, ${username}!`);
});

myEmitter.emit('userLoggedIn', 'Alice');
```

#### Q4: Implement a simple Express.js middleware.
**Question:**
en: Implement a simple Express.js middleware function that logs the HTTP method and URL of incoming requests.
vi: Triển khai một hàm middleware Express.js đơn giản để ghi nhật ký (log) phương thức HTTP và URL của các yêu cầu đến.

**Answer:**
en: A middleware function receives `req`, `res`, and `next`. It logs the details and calls `next()` to pass control to the next middleware.
vi: Một hàm middleware nhận `req`, `res`, và `next`. Nó ghi lại thông tin và gọi `next()` để chuyển quyền kiểm soát sang middleware tiếp theo.
```javascript
const express = require('express');
const app = express();

const requestLogger = (req, res, next) => {
  console.log(`${req.method} ${req.url}`);
  next();
};

app.use(requestLogger);

app.get('/', (req, res) => {
  res.send('Home Page');
});

app.listen(3000);
```

#### Q5: Demonstrate how to parse command-line arguments.
**Question:**
en: Demonstrate how to parse command-line arguments in a Node.js script.
vi: Minh họa cách phân tích (parse) các đối số dòng lệnh (command-line arguments) trong một kịch bản (script) Node.js.

**Answer:**
en: Command-line arguments are stored in the `process.argv` array. The first two elements are the Node executable and script path. We slice from index 2 to get the actual arguments.
vi: Các đối số dòng lệnh được lưu trữ trong mảng `process.argv`. Hai phần tử đầu tiên là trình thực thi Node và đường dẫn kịch bản. Chúng ta cắt từ chỉ số 2 để lấy các đối số thực tế.
```javascript
// Run with: node script.js arg1 arg2
const args = process.argv.slice(2);

if (args.length === 0) {
  console.log('No arguments provided.');
} else {
  args.forEach((val, index) => {
    console.log(`${index}: ${val}`);
  });
}
```