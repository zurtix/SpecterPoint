import { createLazyFileRoute } from '@tanstack/react-router'
import ghosty from "../assets/ghosty.gif"
import poof from "../assets/poof.gif"
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import LoginForm from '@/components/forms/login-form'
import { useState, useEffect } from 'react'
import SetupForm from '@/components/forms/setup-form'
import { invoke } from '@tauri-apps/api/tauri'
import { useToast } from "@/components/ui/use-toast"

export const Route = createLazyFileRoute('/')({
  component: () => (
    <Index />
  )
})

function Index() {

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
          <div className="h-[75px] mt-10" onMouseEnter={() => setImage(poof)} onMouseLeave={() => setImage(ghosty)}>
            <img src={image} width={100} className="opacity-20 m-auto" />
          </div>
          {setupRequired ?
            (
              <div>
                <CardHeader>
                  <CardTitle>
                    Initial Setup
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <SetupForm />
                </CardContent>
              </div>
            )
            :
            (
              <div>
                <CardHeader>
                  <CardTitle>
                    Login
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <LoginForm />
                </CardContent>
              </div>
            )
          }
        </Card>
      </div>
    </div >
  );
}
