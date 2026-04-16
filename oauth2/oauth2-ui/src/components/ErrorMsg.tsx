import { cn } from "@/lib/utils";
import { AlertCircleIcon } from "lucide-react";

import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";

export default function ErrorMsg({
  err = null,
  className = "",
}: {
  err: Error | null;
  className?: string;
}) {
  return (
    <>
      {err && (
        <Alert variant="destructive" className={cn("my-3", className)}>
          <AlertCircleIcon />
          <AlertTitle>有错误发生</AlertTitle>
          <AlertDescription>{err.message}</AlertDescription>
        </Alert>
      )}
    </>
  );
}
