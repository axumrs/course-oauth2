import { Link, useSearchParams } from "react-router-dom";
import { Button } from "./ui/button";
import { Card, CardContent, CardFooter } from "./ui/card";
import { User2 as UserIcon, AppWindowMac as AppIcon } from "lucide-react";
import { useMutation, useQuery } from "@tanstack/react-query";
import { applicationDetailApi } from "@/api/application";
import { findAuthorizeApi, newAuthorizeApi } from "@/api/authorize";
import { useEffect, useMemo } from "react";

export default function OauthAuthorizeConfirm() {
  const [sp, _] = useSearchParams();
  let client_id = sp.get("client_id");
  let scope = sp.get("scope");

  const { data: _appData } = useQuery({
    queryKey: ["app-detail", client_id],
    queryFn: applicationDetailApi(client_id!),
  });
  const { data: _findAuthData } = useQuery({
    queryKey: ["find-authorize", client_id],
    queryFn: () => findAuthorizeApi(client_id!),
  });

  const appData = useMemo(
    () => (_appData ? (_appData as Application) : null),
    [_appData],
  );

  const findAuthData = useMemo(
    () =>
      _findAuthData
        ? (_findAuthData as Authorize & { is_authorized: boolean })
        : null,
    [_findAuthData],
  );

  const isAuthorized = useMemo(
    () => findAuthData?.is_authorized === true && appData,
    [findAuthData, appData],
  );

  const { mutate: newAuthorizeMutate } = useMutation({
    mutationKey: ["new-authorize", client_id],
    mutationFn: newAuthorizeApi({ client_id: client_id!, scope: scope! }),
    onSuccess: (data) => {
      const url = `${appData?.callback_url}?code=${(data as Token).token}`;
      console.log(url);
      return (window.location.href = url);
    },
  });

  useEffect(() => {
    if (isAuthorized) {
      newAuthorizeMutate();
    }
  }, [findAuthData, appData]);

  if (isAuthorized) {
    return (
      <Card className="my-6">
        <CardContent>
          <div>已授权给{appData?.name}，正在跳转</div>
        </CardContent>
      </Card>
    );
  }
  return (
    <>
      <div className="text-2xl text-center my-6 font-semibold">
        授权给{appData?.name}
      </div>
      <Card>
        <CardContent className="space-y-6">
          <div className="flex justify-start items-center gap-x-4">
            <div className="text-gray-600 ">
              <AppIcon size={42} />
            </div>
            <div>
              <div className="font-semibold">应用名称</div>
              <div className="text-xs text-muted-foreground">
                想要访问你的账号
              </div>
            </div>
          </div>
          <div className="flex justify-start items-center gap-x-4">
            <div className="text-gray-600">
              <UserIcon size={42} />
            </div>
            <div>
              <div className="font-semibold">用户个人数据</div>
              <div className="text-xs text-muted-foreground">
                Email 地址（只读）
              </div>
            </div>
          </div>
        </CardContent>
        <CardFooter className="w-full">
          <div className="flex justify-between items-center gap-x-1 w-full">
            <Button variant="outline" asChild className="px-12">
              <Link to="/">取消</Link>
            </Button>
            <Button className="px-12" onClick={() => newAuthorizeMutate()}>
              确认授权
            </Button>
          </div>
        </CardFooter>
      </Card>
    </>
  );
}
