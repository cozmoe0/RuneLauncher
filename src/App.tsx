import React, { useState } from 'react';
import { AppLayout } from './components/layout/AppLayout';
import { Launcher } from './pages/Launcher';

export default function App() {
    const [activeTab, setActiveTab] = useState("accounts");

    return (
        <AppLayout activeTab={activeTab} onTabChange={setActiveTab}>
            <Launcher activeTab={activeTab} />
        </AppLayout>
    );
}