import PageTitle from "@/components/PageTitle";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useQuery } from "@tanstack/react-query";
import { applicationListApi } from "@/api/application";
import ErrorMsg from "@/components/ErrorMsg";
import { useMemo } from "react";

export default function ApplicationList() {
  const hostName = (url: string) => {
    const u = new URL(url);
    return u.hostname;
  };

  const { data, error } = useQuery({
    queryKey: ["appliction-list"],
    queryFn: applicationListApi,
  });

  const appList = useMemo(() => (data ? (data as Application[]) : []), [data]);

  return (
    <>
      <div className="flex justify-between items-center gap-x-3 my-3">
        <PageTitle>应用列表</PageTitle>
        <div>
          <Button asChild>
            <Link to="/application/new">新建应用</Link>
          </Button>
        </div>
      </div>
      <ErrorMsg err={error} className="my-3" />
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>应用名</TableHead>
            <TableHead>主页</TableHead>
            <TableHead>客户ID</TableHead>
            <TableHead>操作</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {appList.map((app) => (
            <TableRow>
              <TableCell>{app.name}</TableCell>
              <TableCell>{hostName(app.homepage_url)}</TableCell>
              <TableCell>
                <Badge variant="secondary">{app.id}</Badge>
              </TableCell>
              <TableCell>
                <div>
                  <Button variant="link" size="sm" className="p-0" asChild>
                    <Link to={`/application/${app.id}`}>详情</Link>
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </>
  );
}
