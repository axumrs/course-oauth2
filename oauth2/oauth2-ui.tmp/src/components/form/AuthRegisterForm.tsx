import { Button } from "@/components/ui/button";
import {
  FieldGroup,
  Field,
  FieldLabel,
  FieldError,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import * as z from "zod";
import { Link } from "react-router-dom";
import { cn } from "@/lib/utils";
import { useMutation } from "@tanstack/react-query";
import { registerApi } from "@/api/auth";
import ErrorMsg from "../ErrorMsg";

const formSchema = z.object({
  username: z.string().min(3, "请输入用户名"),
  email: z.email("请输入正确的邮箱"),
  password: z.string().min(6, "请输入密码"),
  re_password: z.string().min(6, "请再次输入密码"),
});

export default function AuthRegisterForm({
  className = "",
  successCallback = () => {},
}: {
  className?: string;
  successCallback?: () => void;
}) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
      re_password: "",
    },
  });

  const { mutate, error, isPending } = useMutation({
    mutationKey: ["auth-register"],
    mutationFn: registerApi,
    onSuccess: () => {
      successCallback();
    },
  });
  const onSubmit = async (data: z.infer<typeof formSchema>) => {
    mutate(data);
  };

  return (
    <Card className={cn(className)}>
      <CardHeader>
        <CardTitle>用户注册</CardTitle>
      </CardHeader>
      <CardContent>
        <ErrorMsg err={error} />
        <form id="form" onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            <Controller
              name="username"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="username">用户名</FieldLabel>
                  <Input
                    {...field}
                    id="username"
                    aria-invalid={fieldState.invalid}
                    placeholder="请输入用户名"
                    autoComplete="off"
                  />

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              name="email"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="email">邮箱</FieldLabel>
                  <Input
                    {...field}
                    id="email"
                    aria-invalid={fieldState.invalid}
                    placeholder="请输入邮箱"
                    autoComplete="off"
                    type="email"
                  />

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="password"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="password">密码</FieldLabel>
                  <Input
                    {...field}
                    id="password"
                    aria-invalid={fieldState.invalid}
                    placeholder="请输入密码"
                    autoComplete="off"
                    type="password"
                  />

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              name="re_password"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="re_password">重复密码</FieldLabel>
                  <Input
                    {...field}
                    id="re_password"
                    aria-invalid={fieldState.invalid}
                    placeholder="请再次输入密码"
                    autoComplete="off"
                    type="password"
                  />

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>
        </form>
      </CardContent>
      <CardFooter>
        <Field orientation="horizontal">
          <Button type="submit" form="form" disabled={isPending}>
            注册
          </Button>
          <Button type="button" variant="link" asChild>
            <Link to="/auth/login">登录</Link>
          </Button>
        </Field>
      </CardFooter>
    </Card>
  );
}
