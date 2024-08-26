import React, {useState} from "react";
import {hideModal} from "@/app/utils";
import {Project} from "@/app/api/models/project";
import {ProjectState} from "@/app/api/models/project-state";

interface NewProjectModalProps {
    projects: Project[];
    setProjects: React.Dispatch<React.SetStateAction<Project[]>>;
    promotion_id: string;
}

const NewProjectModal: React.FC<NewProjectModalProps> = ({projects, setProjects, promotion_id}) => {
    const [formData, setFormData] = useState({
        name: "",
        description: "",
        end_date: "",
        start_date: "",
        notation_period_duration: "",
    });

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {name, value} = e.target;
        setFormData((prevData) => ({...prevData, [name]: value}));
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            formData.start_date = formData.start_date + ":00";
            formData.end_date = formData.end_date + ":00";
            const notation_period_duration = parseInt(formData.notation_period_duration);

            const response = await fetch(`http://localhost:8080/api/projects/promotion/${promotion_id}`, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify({
                    name: formData.name,
                    description: formData.description,
                    end_date: formData.end_date,
                    start_date: formData.start_date,
                    notation_period_duration
                }),
                credentials: "include",
            });

            if (response.status === 201) {
                const id: string = await response.json();
                const newProject: Project = {
                    id,
                    name: formData.name,
                    description: formData.description,
                    end_date: formData.end_date,
                    start_date: formData.start_date,
                    notation_period_duration: formData.notation_period_duration as unknown as number,
                    promotion_id,
                    state: ProjectState.NotStarted
                }
                setProjects([...projects, newProject]);
                hideModal("new_project_modal");
            } else {
                setError("An error occurred. Please try again later.");
            }
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <dialog id="new_project_modal" className="modal">
            <div className="modal-box">
                <h3 className="font-bold text-lg">Add a student</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p style={{color: "red"}}>{error}</p>
                ): null}
                <form onSubmit={handleSubmit} className={"flex flex-col"}>
                    <input
                        type="text"
                        placeholder="Project name"
                        name="name"
                        id="name"
                        value={formData.name}
                        required
                        maxLength={64}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <input
                        type="text"
                        placeholder="Project description"
                        name="description"
                        id="description"
                        value={formData.description}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <input
                        type="datetime-local"
                        placeholder="Start date"
                        name="start_date"
                        id="start_date"
                        value={formData.start_date}
                        required
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <input
                        type="datetime-local"
                        placeholder="End date"
                        name="end_date"
                        id="end_date"
                        value={formData.end_date}
                        required
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <input
                        type="number"
                        placeholder="Notation period duration (in days)"
                        name="notation_period_duration"
                        id="notation_period_duration"
                        value={formData.notation_period_duration}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <div className="my-4 flex justify-end w-full">
                        <button type={"submit"}
                                className={"flex- rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Submit
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    )
}

export default NewProjectModal;