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
import { useState } from 'react'
import SetupForm from '@/components/forms/setup-form'

export const Route = createLazyFileRoute('/')({
  component: () => (
    <Index />
  )
})

function Index() {

  const firstLaunch: boolean = false; // TODO: Obtain if first launched from the database backend clientside
  const [image, setImage] = useState(ghosty);

  return (
    <div>
      <div className="flex items-center w-full h-screen">
        <Card className="w-[400px] m-auto" >
          <div className="h-[75px] mt-10" onMouseEnter={() => setImage(poof)} onMouseLeave={() => setImage(ghosty)}>
            <img src={image} width={100} className="opacity-20 m-auto" />
          </div>
          {firstLaunch ?
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
