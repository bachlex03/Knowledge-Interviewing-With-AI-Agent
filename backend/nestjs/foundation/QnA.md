# NestJS Foundation Q&A

### Level 1: Remembering

#### Q1: What is NestJS?

**Question:**
en: What is NestJS?
vi: NestJS là gì?

**Answer:**
en: NestJS is a progressive Node.js framework for building efficient and scalable server-side applications. It uses TypeScript by default and provides modules, controllers, providers, and dependency injection.
vi: NestJS là một framework Node.js theo hướng progressive để xây dựng các ứng dụng server-side hiệu quả và có khả năng mở rộng. Nó sử dụng TypeScript mặc định và cung cấp modules, controllers, providers, cùng cơ chế dependency injection.

#### Q2: What are modules used for in NestJS?

**Question:**
en: What are modules used for in NestJS?
vi: Modules được dùng để làm gì trong NestJS?

**Answer:**
en: Modules group related controllers and providers into a single feature boundary. They help organize the application into manageable parts and make dependency management clearer.
vi: Modules gom các controller và provider liên quan vào một ranh giới tính năng duy nhất. Chúng giúp tổ chức ứng dụng thành các phần dễ quản lý hơn và làm cho việc quản lý phụ thuộc rõ ràng hơn.

#### Q3: What is the role of a controller?

**Question:**
en: What is the role of a controller?
vi: Vai trò của controller là gì?

**Answer:**
en: A controller receives incoming requests, calls the appropriate service or provider, and returns a response to the client.
vi: Controller nhận request đầu vào, gọi service hoặc provider phù hợp, và trả response về cho client.

#### Q4: What is the role of a provider?

**Question:**
en: What is the role of a provider?
vi: Vai trò của provider là gì?

**Answer:**
en: A provider is a class that can be injected as a dependency, most often a service containing reusable business logic.
vi: Provider là một class có thể được inject như một dependency, thường là service chứa logic nghiệp vụ có thể tái sử dụng.

#### Q5: What is dependency injection in NestJS?

**Question:**
en: What is dependency injection in NestJS?
vi: Dependency injection trong NestJS là gì?

**Answer:**
en: Dependency injection is a pattern where NestJS creates and supplies dependencies automatically instead of requiring manual instantiation. This improves testability and separation of concerns.
vi: Dependency injection là một mẫu thiết kế trong đó NestJS tự động tạo và cung cấp các dependency thay vì phải khởi tạo thủ công. Điều này giúp tăng khả năng kiểm thử và tách biệt trách nhiệm.

---

### Level 2: Understanding

#### Q1: Explain the relationship between modules, controllers, and providers.

**Question:**
en: Explain the relationship between modules, controllers, and providers.
vi: Giải thích mối quan hệ giữa modules, controllers và providers.

**Answer:**
en: Modules define the feature boundary, controllers expose endpoints, and providers contain the business logic that controllers use. Together they create a clean application structure.
vi: Modules xác định ranh giới tính năng, controllers công khai các endpoint, còn providers chứa logic nghiệp vụ mà controllers sử dụng. Ba thành phần này kết hợp để tạo ra cấu trúc ứng dụng rõ ràng.

#### Q2: Why is NestJS a good fit for TypeScript projects?

**Question:**
en: Why is NestJS a good fit for TypeScript projects?
vi: Vì sao NestJS phù hợp với các dự án TypeScript?

**Answer:**
en: NestJS is designed around TypeScript, so it supports decorators, interfaces, type safety, and better tooling out of the box. This makes large backend applications easier to maintain.
vi: NestJS được thiết kế xoay quanh TypeScript nên hỗ trợ decorators, interfaces, type safety và bộ công cụ tốt ngay từ đầu. Điều này giúp các ứng dụng backend lớn dễ bảo trì hơn.

#### Q3: What problem does dependency injection solve?

**Question:**
en: What problem does dependency injection solve?
vi: Dependency injection giải quyết vấn đề gì?

**Answer:**
en: It removes the need to manually create dependencies everywhere, which reduces coupling, improves reuse, and makes testing easier.
vi: Nó loại bỏ việc phải tự tạo dependency ở nhiều nơi, từ đó giảm độ phụ thuộc chặt, tăng khả năng tái sử dụng và giúp việc test dễ hơn.

#### Q4: How do DTOs help in NestJS applications?

**Question:**
en: How do DTOs help in NestJS applications?
vi: DTO giúp ích gì trong các ứng dụng NestJS?

**Answer:**
en: DTOs define the shape of data moving between layers. They make request validation, documentation, and transformation more predictable.
vi: DTO định nghĩa cấu trúc dữ liệu di chuyển giữa các tầng. Chúng làm cho validation request, tài liệu hóa và chuyển đổi dữ liệu trở nên dễ dự đoán hơn.

#### Q5: Why should controllers stay thin?

**Question:**
en: Why should controllers stay thin?
vi: Tại sao controller nên giữ cho gọn?

**Answer:**
en: Controllers should focus on HTTP concerns such as routing, request parsing, and response handling. Keeping business logic in services makes the code easier to test and maintain.
vi: Controller nên tập trung vào các vấn đề HTTP như routing, xử lý request và response. Giữ logic nghiệp vụ trong service giúp code dễ test và bảo trì hơn.

---

### Level 3: Applying

#### Q1: Show how to create a basic NestJS feature module.

**Question:**
en: Show how to create a basic NestJS feature module.
vi: Hãy minh họa cách tạo một feature module cơ bản trong NestJS.

**Answer:**
en: Create a module to register the controller and provider, place the business logic in a service, and expose an endpoint in the controller.
vi: Tạo một module để đăng ký controller và provider, đặt logic nghiệp vụ trong service, và expose endpoint trong controller.

```ts
// users.service.ts
import { Injectable } from '@nestjs/common';

@Injectable()
export class UsersService {
  findAll() {
    return [{ id: 1, name: 'Alice' }];
  }
}

// users.controller.ts
import { Controller, Get } from '@nestjs/common';
import { UsersService } from './users.service';

@Controller('users')
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  @Get()
  findAll() {
    return this.usersService.findAll();
  }
}

// users.module.ts
import { Module } from '@nestjs/common';
import { UsersController } from './users.controller';
import { UsersService } from './users.service';

@Module({
  controllers: [UsersController],
  providers: [UsersService],
})
export class UsersModule {}
```

#### Q2: How would you validate incoming request data in NestJS?

**Question:**
en: How would you validate incoming request data in NestJS?
vi: Bạn sẽ validate dữ liệu request đầu vào trong NestJS như thế nào?

**Answer:**
en: Use DTO classes together with validation pipes so invalid input is rejected before it reaches business logic.
vi: Sử dụng DTO class kết hợp với validation pipe để dữ liệu không hợp lệ bị loại bỏ trước khi đi vào logic nghiệp vụ.

```ts
import { IsEmail, IsString, MinLength } from 'class-validator';

export class CreateUserDto {
  @IsEmail()
  email: string;

  @IsString()
  @MinLength(2)
  name: string;
}
```

#### Q3: How do you inject a service into a controller?

**Question:**
en: How do you inject a service into a controller?
vi: Làm thế nào để inject một service vào controller?

**Answer:**
en: Register the service in the module providers array and declare it in the controller constructor. NestJS will resolve it automatically.
vi: Đăng ký service trong mảng providers của module và khai báo nó trong constructor của controller. NestJS sẽ tự động resolve dependency này.

```ts
constructor(private readonly usersService: UsersService) {}
```

#### Q4: How would you add a guard to protect an endpoint?

**Question:**
en: How would you add a guard to protect an endpoint?
vi: Bạn sẽ thêm guard để bảo vệ một endpoint như thế nào?

**Answer:**
en: Create a guard that checks the request context, then apply it at the controller or route level to block unauthorized access.
vi: Tạo một guard kiểm tra context của request, sau đó áp dụng nó ở cấp controller hoặc route để chặn truy cập không hợp lệ.

```ts
import { CanActivate, ExecutionContext, Injectable } from '@nestjs/common';

@Injectable()
export class AuthGuard implements CanActivate {
  canActivate(context: ExecutionContext): boolean {
    const request = context.switchToHttp().getRequest();
    return Boolean(request.headers.authorization);
  }
}
```

#### Q5: How would you use a pipe to transform route parameters?

**Question:**
en: How would you use a pipe to transform route parameters?
vi: Bạn sẽ dùng pipe để chuyển đổi tham số route như thế nào?

**Answer:**
en: Use built-in pipes like `ParseIntPipe` to convert string parameters into the correct type before the handler runs.
vi: Sử dụng các pipe có sẵn như `ParseIntPipe` để chuyển tham số dạng chuỗi sang đúng kiểu dữ liệu trước khi handler chạy.

```ts
@Get(':id')
findOne(@Param('id', ParseIntPipe) id: number) {
  return this.usersService.findOne(id);
}
```
