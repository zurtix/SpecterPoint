import { Outlet, createFileRoute } from '@tanstack/react-router'
import { Navigation } from "@/components/navigation"
import { EventViewer } from '@/components/eventviewer'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'

export const Route = createFileRoute('/_auth')({
  component: () => (
    <div className="h-[calc(100vh-59px)] w-full flex flex-row">
      <div className="w-full">
        <ResizablePanelGroup direction="vertical">
          <ResizablePanel>
            <Outlet />
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel>
            <EventViewer />
          </ResizablePanel>
        </ResizablePanelGroup>
        <Navigation />
      </div>
    </div>
  )
})
