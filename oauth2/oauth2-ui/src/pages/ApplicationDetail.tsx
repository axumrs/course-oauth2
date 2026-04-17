import PageTitle from "@/components/PageTitle";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import {
  KeyRound as KeyIcon,
  Check as IsNewIcon,
  Copy as CopyIcon,
} from "lucide-react";
import { Link, useParams } from "react-router-dom";
import { useMutation, useQuery } from "@tanstack/react-query";
import {
  applicationDetailApi,
  applicationSecretListApi,
  newApplicationSecretApi,
} from "@/api/application";
import { useMemo, useState } from "react";
import ErrorMsg from "@/components/ErrorMsg";

export default function ApplicationDetail() {
  const { id } = useParams();
  const { data, error } = useQuery({
    queryKey: ["application", id],
    queryFn: applicationDetailApi(id!),
  });
  const appData = useMemo(() => (data ? (data as Application) : null), [data]);

  const [newSecret, setNewSecret] = useState<ApplicationSecret | null>(null);

  const { data: fetchSecretList, error: secretError } = useQuery({
    queryKey: ["application-secret", id],
    queryFn: applicationSecretListApi(id!),
  });

  const secretList = useMemo(() => {
    const foo = fetchSecretList ? (fetchSecretList as ApplicationSecret[]) : [];

    return newSecret ? [newSecret, ...foo] : [...foo];
  }, [fetchSecretList, newSecret]);

  const { mutate, error: newSecretError } = useMutation({
    mutationFn: newApplicationSecretApi,
    mutationKey: ["application-secret-new"],
    onSuccess: (data) => {
      setNewSecret({ ...(data as ApplicationSecret), is_new: true });
    },
  });
  return (
    <>
      <ErrorMsg err={error} className="my-3" />
      <ErrorMsg err={secretError} className="my-3" />
      <ErrorMsg err={newSecretError} className="my-3" />
      {appData && (
        <>
          <div className="flex justify-between items-center gap-x-3 my-3">
            <PageTitle>{appData.name}</PageTitle>
            <div>
              <Button asChild>
                <Link to="/application">应用列表</Link>
              </Button>
            </div>
          </div>
          <div className="space-y-4">
            <Card>
              <CardContent className="flex justify-between items-center gap-x-3">
                <div>0 个用户</div>
                <div>
                  <Button variant="destructive">移除所有用户令牌</Button>
                </div>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>客户ID</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="font-mono">{appData.id}</div>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>应用URL</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                <div>
                  主页 URL：
                  <span className="font-mono">{appData.homepage_url}</span>
                </div>
                <div>
                  回调 URL：
                  <span className="font-mono">{appData.callback_url}</span>
                </div>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle className="flex justify-between items-center gap-x-3">
                  <div>密钥</div>
                  <div>
                    <Button
                      variant="secondary"
                      onClick={() => {
                        if (window.confirm("确定生成新密钥吗？")) {
                          mutate({ application_id: appData.id });
                        }
                      }}
                    >
                      生成新密钥
                    </Button>
                  </div>
                </CardTitle>
              </CardHeader>
              <CardContent>
                {secretList &&
                  secretList.map((item) => (
                    <SecretKeyItem key={item.id} item={item} />
                  ))}
              </CardContent>
            </Card>
          </div>
        </>
      )}
    </>
  );
}

function SecretKeyItem({ item }: { item: ApplicationSecret }) {
  return (
    <>
      {item.is_new && (
        <Alert className=" border-sky-200 bg-sky-50 text-sky-900 dark:border-sky-900 dark:bg-sky-950 dark:text-sky-50 py-6 px-4">
          <AlertDescription>
            请确保你已经保存好你新创建的密钥，之后你将无法再次看到这个密钥。
          </AlertDescription>
        </Alert>
      )}

      <div className="flex justify-between items-center gap-x-3 bg-green-50/30 border rounded p-3 my-3">
        <div className="flex items-center gap-x-3">
          <div className="flex flex-col items-center justify-center gap-y-1">
            <KeyIcon size={32} />
            <Badge variant="outline">客户密钥</Badge>
          </div>
          <div className="flex items-center gap-x-2">
            {item.is_new && (
              <div className="text-green-600">
                <IsNewIcon />
              </div>
            )}
            <div>{item.secret}</div>
            {item.is_new && (
              <div>
                <Button variant="ghost">
                  <CopyIcon />
                </Button>
              </div>
            )}
          </div>
        </div>
        <div>
          <Button variant="destructive">删除</Button>
        </div>
      </div>
    </>
  );
}
