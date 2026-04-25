import { Card } from "@lib/ui/components/design";
import { PageLayout } from "@lib/ui/components/layout/Page";

/**
 * Assets uploading Page
 */
function UploadPage() {
    return (
        <PageLayout variant="centered" className="h-screen">
            <Card className="p-4">
                <h1>Uploading Page</h1>
            </Card>
        </PageLayout>
    );
}

export { UploadPage };
