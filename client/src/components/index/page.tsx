import ghosty from "@/assets/ghosty.gif"
import poof from "@/assets/poof.gif"
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import LoginForm from '@/components/index/form-login'
import SetupForm from '@/components/index/form-setup'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useToast } from "@/components/ui/use-toast"

export function Index() {
  const { toast } = useToast()
  const [setupRequired, setSetupRequired] = useState(false);
  const [image, setImage] = useState(ghosty);

  useEffect(() => {
    invoke<boolean>("is_setup_required")
      .then((setup: boolean) => setSetupRequired(setup))
      .catch((err: string) => toast({
        variant: "destructive",
        title: err,
      })
      )
  }, [setupRequired])

  return (
    <div>
      <div className="flex items-center w-full h-screen">
        <Card className="w-[400px] m-auto" >
          <div
            className="h-[75px] mt-10"
            onMouseEnter={() => setImage(poof)}
            onMouseLeave={() => setImage(ghosty)}>
            <img src={image} width={100} className="opacity-20 m-auto" />
          </div>
          <CardHeader>
            <CardTitle>
              {setupRequired ? "Initial Setup" : "Login"}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {setupRequired ? <SetupForm /> : <LoginForm />}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
