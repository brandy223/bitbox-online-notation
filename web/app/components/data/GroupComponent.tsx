import React, {useState} from "react";
import {ProjectGroup} from "@/app/api/models/project-group";
import MinimalStudentComponent from "@/app/components/data/MinimalStudentComponent";
import {ProjectState} from "@/app/api/models/project-state";

async function updateGroupMark(group_id: string, mark: number) {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    await fetch(`${api_url}/groups/${group_id}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ mark: mark }),
    });
}

const GroupComponent: React.FC<{
    info: ProjectGroup;
    withMarks: boolean;
    onDelete: (id: string) => void;
    projectState: ProjectState;
}> = ({ info, withMarks, onDelete, projectState }) => {
    const [groupMark, setGroupMark] = useState<number>(info.group.mark || 0);
    const [editMode, setEditMode] = useState<boolean>(false);
    const [markSubmitting, setMarkSubmitting] = useState<boolean>(false);
    const [markError, setMarkError] = useState<string | null>(null);

    const handleMarkSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setMarkSubmitting(true);
        setMarkError(null);

        try {
            await updateGroupMark(info.group.id, groupMark);
            setEditMode(false);
        } catch (error) {
            setMarkError("Failed to update the mark. Please try again.");
        } finally {
            setMarkSubmitting(false);
        }
    };

    return (
        <div className="relative px-4 py-2 border rounded-lg shadow-sm hover:shadow-md transition-shadow group flex-col">
            <div className="flex-row">
                <p>{info.group.name.toUpperCase()}</p>
                {withMarks && !editMode && <p>{groupMark}</p>}
            </div>
            <div>
                {info.students.map((student) => (
                    <MinimalStudentComponent
                        key={student.student.id}
                        student_details={student}
                        withMark={withMarks}
                        group_id={info.group.id}
                        student_id={student.student.id}
                    />
                ))}
            </div>

            {projectState === ProjectState.Finished && (
                <div className="mt-4">
                    {editMode ? (
                        <form onSubmit={handleMarkSubmit}>
                            <input
                                type="number"
                                value={groupMark}
                                onChange={(e) => setGroupMark(Number(e.target.value))}
                                min="0"
                                max="20"
                                required
                                className="border rounded px-2 py-1"
                            />
                            <button type="submit" className="ml-2 px-4 py-1 bg-blue-500 text-white rounded" disabled={markSubmitting}>
                                {markSubmitting ? "Submitting..." : "Submit"}
                            </button>
                            <button
                                type="button"
                                className="ml-2 px-4 py-1 bg-gray-500 text-white rounded"
                                onClick={() => setEditMode(false)}
                                disabled={markSubmitting}
                            >
                                Cancel
                            </button>
                            {markError && <p className="text-red-500">{markError}</p>}
                        </form>
                    ) : (
                        <button
                            className="mt-2 px-4 py-1 bg-blue-500 text-white rounded"
                            onClick={() => setEditMode(true)}
                        >
                            Edit Mark
                        </button>
                    )}
                </div>
            )}
        </div>
    );
};

export default GroupComponent;
