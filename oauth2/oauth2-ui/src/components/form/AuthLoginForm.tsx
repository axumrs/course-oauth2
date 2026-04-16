import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import * as z from "zod";
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
import { cn } from "@/lib/utils";
import { Link } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { loginApi } from "@/api/auth";
import ErrorMsg from "../ErrorMsg";

const formSchema = z.object({
  username: z.string().min(3, "请输入用户名"),
  password: z.string().min(6, "请输入密码"),
});

export default function AuthLoginForm({
  classname = "",
}: {
  classname?: string;
}) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const { mutate, error, isPending } = useMutation({
    mutationKey: ["auth-login"],
    mutationFn: loginApi,
    onSuccess: (data) => {
      window.sessionStorage.setItem("token", JSON.stringify(data));
    },
  });

  const onSubmit = async (data: z.infer<typeof formSchema>) => {
    mutate(data);
  };

  return (
    <Card className={cn(classname)}>
      <CardHeader>
        <CardTitle>用户登录</CardTitle>
      </CardHeader>
      <CardContent>
        <ErrorMsg err={error} />
        <form id="form" onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            {/* 字段开始 */}
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
            {/* 字段结束 */}
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
          </FieldGroup>
        </form>
      </CardContent>
      <CardFooter>
        <Field orientation="horizontal">
          <Button type="submit" form="form" disabled={isPending}>
            登录
          </Button>
          <Button type="button" variant="link" asChild>
            <Link to="/auth/register">注册</Link>
          </Button>
        </Field>
      </CardFooter>
    </Card>
  );
}
