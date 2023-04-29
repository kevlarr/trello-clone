import { useRouter } from 'next/router'

import api from '@/lib/api'
import css from './Home.module.css'

export default function Home() {
    const router = useRouter()

    function createWorkspace() {
        api
            .post('workspaces')
            .then(({ identifier}) => router.push(`workspaces/${identifier}`))
    }

    return (
        <div className={css.home}>
            <h2 className={css.welcome}>
                a
                <span className={css.hello}> hello </span>
                from treclo!
            </h2>

            <p className={css.caption}>
                Workspaces give you places to make things like boards and cards... and magic!
                Create a new one or, if you're really lucky, get a friend to share a workspace with you.
            </p>
            <div className={css.create}>
                <button className={css.button} onClick={createWorkspace}>
                    Create workspace
                </button>
            </div>
        </div>
    )
}