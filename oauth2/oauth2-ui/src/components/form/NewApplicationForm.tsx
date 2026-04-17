import { Button } from "@/components/ui/button";
import {
  FieldGroup,
  Field,
  FieldLabel,
  FieldError,
  FieldDescription,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Card, CardContent, CardFooter } from "@/components/ui/card";

import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import * as z from "zod";
import { cn } from "@/lib/utils";
import { useMutation } from "@tanstack/react-query";
import { newApplicationApi } from "@/api/application";
import ErrorMsg from "../ErrorMsg";
import { Link } from "react-router-dom";

const formSchema = z.object({
  name: z.string().min(3, "请输入应用名称"),
  homepage: z.url("请输入正确的网址"),
  desc: z.string(),
  callback: z.url("请输入正确的网址"),
});

export default function NewApplicationForm({
  className = "",
}: {
  className?: string;
}) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      homepage: "",
      desc: "",
      callback: "",
    },
  });

  const { mutate, error, isPending } = useMutation({
    mutationKey: ["appliction-new"],
    mutationFn: newApplicationApi,
    onSuccess: () => {},
  });

  const onSubmit = async (data: z.infer<typeof formSchema>) => {
    mutate(data);
  };
  return (
    <Card className={cn(className)}>
      {/* <CardHeader>
        <CardTitle>创建新的 OAuth 应用</CardTitle>
        <CardDescription>填写以下表单，提交你的申请。</CardDescription>
      </CardHeader> */}
      <CardContent>
        <ErrorMsg err={error} />
        <form id="form" onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            <Controller
              name="name"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="name">应用名称</FieldLabel>
                  <Input
                    {...field}
                    id="name"
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                  />
                  <FieldDescription>
                    设置让用户更加信任的应用名称。
                  </FieldDescription>

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="desc"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="desc">应用描述</FieldLabel>
                  <Textarea
                    {...field}
                    id="desc"
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    placeholder="可选的应用描述"
                  />
                  <FieldDescription>
                    此信息将显示给您应用程序的所有用户。
                  </FieldDescription>

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="homepage"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="homepage">主页 URL</FieldLabel>
                  <Input
                    {...field}
                    id="homepage"
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    type="url"
                  />
                  <FieldDescription>应用程序主页的完整 URL。</FieldDescription>

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="callback"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="callback">授权回调 URL</FieldLabel>
                  <Input
                    {...field}
                    id="callback"
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    type="url"
                  />
                  <FieldDescription>您应用程序的回调 URL。</FieldDescription>

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
            提交申请
          </Button>
          <Button type="button" variant="link" asChild>
            <Link to="/application">取消</Link>
          </Button>
        </Field>
      </CardFooter>
    </Card>
  );
}
